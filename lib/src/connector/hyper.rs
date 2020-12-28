use std::io::{Cursor, Read};
use std::path::Path;
use std::pin::Pin;
use std::str::FromStr;

use bytes::Bytes;
use futures::{Future, FutureExt};
use hyper::{
    body::to_bytes,
    client::{connect::Connect, Client},
    header::CONTENT_TYPE,
    http::Error as HttpError,
    Method, Request, Uri,
};
#[cfg(feature = "rustls")]
use hyper_rustls::HttpsConnector;
#[cfg(feature = "openssl")]
use hyper_tls::HttpsConnector;
use multipart::client::lazy::Multipart;
use telegram_bot_raw::{
    Body as TelegramBody, HttpRequest, HttpResponse, Method as TelegramMethod, MultipartValue, Text,
};

use super::Connector;
use crate::errors::{Error, ErrorKind};

#[derive(Debug)]
pub struct HyperConnector<C>(Client<C>);

enum MultipartTemporaryValue {
    Text(Text),
    Data { file_name: Text, data: Bytes },
}

impl<C> HyperConnector<C> {
    pub fn new(client: Client<C>) -> Self {
        HyperConnector(client)
    }
}

impl<C: Connect + std::fmt::Debug + 'static + Clone + Send + Sync> Connector for HyperConnector<C> {
    fn request(
        &self,
        token: &str,
        req: HttpRequest,
    ) -> Pin<Box<dyn Future<Output = Result<HttpResponse, Error>> + Send>> {
        let uri = Uri::from_str(&req.url.url(token));
        let client = self.0.clone();

        let future = async move {
            let uri = uri.map_err(HttpError::from).map_err(ErrorKind::from)?;

            let method = match req.method {
                TelegramMethod::Get => Method::GET,
                TelegramMethod::Post => Method::POST,
            };

            let mut http_request = Request::builder().method(method).uri(uri);

            let request = match req.body {
                TelegramBody::Empty => http_request.body(Into::<hyper::Body>::into(vec![])),
                TelegramBody::Json(body) => {
                    let content_type = "application/json"
                        .parse()
                        .map_err(HttpError::from)
                        .map_err(ErrorKind::from)?;
                    http_request
                        .headers_mut()
                        .map(move |headers| headers.insert(CONTENT_TYPE, content_type));
                    http_request.body(Into::<hyper::Body>::into(body))
                }
                TelegramBody::Multipart(parts) => {
                    let mut fields = Vec::new();
                    for (key, value) in parts {
                        match value {
                            MultipartValue::Text(text) => {
                                fields.push((key, MultipartTemporaryValue::Text(text)))
                            }
                            MultipartValue::Path { file_name, path } => {
                                let file_name = file_name
                                    .or_else(|| {
                                        AsRef::<Path>::as_ref(&path)
                                            .file_name()
                                            .and_then(|s| s.to_str())
                                            .map(Into::into)
                                    })
                                    .ok_or(ErrorKind::InvalidMultipartFilename)?;

                                let data = tokio::fs::read(path).await.map_err(ErrorKind::from)?;
                                fields.push((
                                    key,
                                    MultipartTemporaryValue::Data {
                                        file_name,
                                        data: data.into(),
                                    },
                                ))
                            }
                            MultipartValue::Data { file_name, data } => fields
                                .push((key, MultipartTemporaryValue::Data { file_name, data })),
                        }
                    }

                    let mut prepared = {
                        let mut part = Multipart::new();
                        for (key, value) in &fields {
                            match value {
                                MultipartTemporaryValue::Text(text) => {
                                    part.add_text(*key, text.as_str());
                                }
                                MultipartTemporaryValue::Data { file_name, data } => {
                                    part.add_stream(
                                        *key,
                                        Cursor::new(data),
                                        Some(file_name.as_str()),
                                        None,
                                    );
                                }
                            }
                        }
                        part.prepare().map_err(|err| err.error)
                    }
                    .map_err(ErrorKind::from)?;

                    let boundary = prepared.boundary();

                    let content_type =
                        format!("multipart/form-data;boundary={bound}", bound = boundary)
                            .parse()
                            .map_err(HttpError::from)
                            .map_err(ErrorKind::from)?;
                    if let Some(headers) = http_request.headers_mut() {
                        headers.insert(CONTENT_TYPE, content_type);
                    }

                    let mut bytes = Vec::new();
                    prepared.read_to_end(&mut bytes).map_err(ErrorKind::from)?;
                    http_request.body(bytes.into())
                }
                body => panic!("Unknown body type {:?}", body),
            }
            .map_err(ErrorKind::from)?;

            let response = client.request(request).await.map_err(ErrorKind::from)?;
            let whole_chunk = to_bytes(response.into_body()).await;

            let body = whole_chunk
                .iter()
                .fold(vec![], |mut acc, chunk| -> Vec<u8> {
                    acc.extend_from_slice(&chunk);
                    acc
                });

            Ok::<HttpResponse, Error>(HttpResponse { body: Some(body) })
        };

        future.boxed()
    }
}

pub fn default_connector() -> Result<Box<dyn Connector>, Error> {
    #[cfg(feature = "rustls")]
    let connector = HttpsConnector::new();

    #[cfg(feature = "openssl")]
    let connector = HttpsConnector::new();

    Ok(Box::new(HyperConnector::new(
        Client::builder().build(connector),
    )))
}
