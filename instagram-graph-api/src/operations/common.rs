use http_api_client_endpoint::{
    http::{Error as HttpError, StatusCode},
    Body,
};
use serde_json::Error as SerdeJsonError;
use url::ParseError as UrlParseError;

use crate::objects::ResponseBodyErrJson;

//
#[derive(Debug, Clone)]
pub enum EndpointRet<T> {
    Ok(T),
    Other((StatusCode, Result<ResponseBodyErrJson, Body>)),
}

//
#[derive(Debug)]
pub enum EndpointError {
    MakeRequestUrlFailed(UrlParseError),
    MakeRequestFailed(HttpError),
    DeResponseBodyOkJsonFailed(SerdeJsonError),
    Other(Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl core::fmt::Display for EndpointError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for EndpointError {}
