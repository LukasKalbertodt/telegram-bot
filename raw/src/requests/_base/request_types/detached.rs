use crate::requests::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct DetachedRequestType;

impl RequestType for DetachedRequestType {
    type Options = ();
    type Request = Result<HttpRequest, Error>;

    fn serialize(_options: Self::Options, request: &Self::Request) -> Result<HttpRequest, Error> {
        match request {
            Ok(ref req) => Ok(req.clone()),
            Err(ref err) => Err(ErrorKind::DetachedError(err.to_string()).into()),
        }
    }
}
