use serde::de::{Deserialize, Deserializer, Error};

use crate::types::*;

/// All API responses are from this type. Mostly used internal.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum ResponseWrapper<T> {
    /// Request was successful.
    Success {
        /// Response result.
        result: T,
    },
    /// Request was unsuccessful.
    Error {
        /// Human-readable description of the result.
        description: String,
        /// Contains information about why a request was unsuccessful.
        parameters: Option<ResponseParameters>,
    },
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for ResponseWrapper<T> {
    fn deserialize<D>(deserializer: D) -> Result<ResponseWrapper<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: RawResponse<T> = Deserialize::deserialize(deserializer)?;
        match (raw.ok, raw.description, raw.result) {
            (false, Some(description), None) => Ok(ResponseWrapper::Error {
                description,
                parameters: raw.parameters,
            }),
            (true, None, Some(result)) => Ok(ResponseWrapper::Success { result }),
            _ => Err(D::Error::custom("ambiguous response")),
        }
    }
}

/// Directly mapped telegram API response.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize)]
pub struct RawResponse<T> {
    /// If ‘ok’ equals true, the request was successful.
    ok: bool,
    /// Human-readable description of the result.
    description: Option<String>,
    /// Result of the query.
    result: Option<T>,
    /// Information about why a request was unsuccessful.
    parameters: Option<ResponseParameters>,
}

/// Contains information about why a request was unsuccessful.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize)]
pub struct ResponseParameters {
    /// The group has been migrated to a supergroup with the specified identifier.
    pub migrate_to_chat_id: Option<Integer>,
    /// In case of exceeding flood control, the number of seconds left to wait
    /// before the request can be repeated.
    pub retry_after: Option<Integer>,
}
