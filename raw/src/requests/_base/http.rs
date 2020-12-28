use std::fmt;

use bytes::Bytes;

use crate::types::Text;
use crate::url::telegram_api_url;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum RequestUrl {
    Method(&'static str),
}

impl RequestUrl {
    pub fn method(method: &'static str) -> Self {
        RequestUrl::Method(method)
    }

    pub fn url(&self, token: &str) -> String {
        match self {
            RequestUrl::Method(method) => format!("{}bot{}/{}", telegram_api_url(), token, method),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Method {
    Get,
    Post,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum MultipartValue {
    Text(Text),
    Path { path: Text, file_name: Option<Text> },
    Data { file_name: Text, data: Bytes },
}

pub type Multipart = Vec<(&'static str, MultipartValue)>;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[non_exhaustive]
pub enum Body {
    Empty,
    Multipart(Multipart),
    Json(String),
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Body::Empty => "<empty body>".fmt(f),
            Body::Multipart(multipart) => write!(f, "{:?}", multipart),
            Body::Json(s) => s.fmt(f),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct HttpRequest {
    pub url: RequestUrl,
    pub method: Method,
    pub body: Body,
}

impl HttpRequest {
    pub fn name(&self) -> &'static str {
        match self.url {
            RequestUrl::Method(method) => method,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct HttpResponse {
    pub body: Option<Vec<u8>>,
}
