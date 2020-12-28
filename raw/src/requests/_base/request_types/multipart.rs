use crate::requests::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct MultipartRequestType<Request> {
    phantom: ::std::marker::PhantomData<Request>,
}

pub trait ToMultipartValue {
    fn to_multipart_value(&self) -> MultipartValue;
}

pub trait ToMultipart {
    fn to_multipart(&self) -> Result<Multipart, Error>;
}

impl<Request: ToMultipart> RequestType for MultipartRequestType<Request> {
    type Options = RequestUrl;
    type Request = Request;

    fn serialize(url: Self::Options, request: &Self::Request) -> Result<HttpRequest, Error> {
        let multipart = request.to_multipart()?;

        Ok(HttpRequest {
            url,
            method: Method::Post,
            body: Body::Multipart(multipart),
        })
    }
}

#[macro_export]
macro_rules! multipart_map {
    ($self:expr, $( ( $($opts:tt)* ) ; )*) => {
        let mut result = Vec::new();
        $(
            multipart_field!($self, result, $($opts)*);
        )*
        Ok(result)
    }
}

macro_rules! multipart_field {
    ($self:expr, $result:expr, $field:ident($type:ident)) => {{
        let value = &$self.$field;
        multipart_field!($self, $result, $field ($type) => value);
    }};

    ($self:expr, $result:expr, $field:ident($type:ident), $($t:tt)*) => {{
        let value = &$self.$field;
        multipart_field!($self, $result, $field ($type) => value, $($t)*);
    }};

    ($self:expr, $result:expr, $field:ident($type:ident) => $val:expr,skip_if $cond:expr) => {{
        if *$cond {
            multipart_field!($self, $result, $field ($type) => $val);
        }
    }};

    ($self:expr, $result:expr, $field:ident($type:ident) => $val:expr,optional) => {{
        let value = $val.as_ref();
        if let Some(value) = value {
            multipart_field!($self, $result, $field ($type) => value);
        }
    }};

    ($self:expr, $result:expr, $field:ident($type:ident) => $val:expr,when_true) => {{
        let value = $val;
        multipart_field!($self, $result, $field ($type) => value, skip_if value);
    }};

    ($self:expr, $result:expr, $field:ident(text) => $val:expr) => {{
        let value = MultipartValue::Text($val.to_string().into());
        $result.push((stringify!($field), value));
    }};

    ($self:expr, $result:expr, $field:ident(json) => $val:expr) => {{
        let s = ::serde_json::to_string($val).map_err(ErrorKind::from)?;
        let value = MultipartValue::Text(s.into());
        $result.push((stringify!($field), value));
    }};
    ($self:expr, $result:expr, $field:ident(raw) => $val:expr) => {{
        let value = $val.to_multipart_value();
        $result.push((stringify!($field), value));
    }};
}
