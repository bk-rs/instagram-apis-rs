use core::fmt;

use http_api_client_endpoint::{
    http::{Error as HttpError, StatusCode},
    Body, Response,
};
use percent_encoding::{AsciiSet, NON_ALPHANUMERIC};
use serde::de::DeserializeOwned;
use serde_json::Error as SerdeJsonError;
use url::ParseError as UrlParseError;

pub const URL_PERCENT_ENCODE_ASCII_SET: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'_')
    .remove(b'.')
    .remove(b'(')
    .remove(b')');

pub const BASE_URL: &str = "https://graph.instagram.com";
pub const API_VERSION: &str = "v15.0";

use crate::objects::ResponseErrorBody;

//
//
//
#[derive(Debug, Clone)]
pub enum EndpointRet<T>
where
    T: fmt::Debug + Clone,
{
    Ok(T),
    Other((StatusCode, Result<ResponseErrorBody, Body>)),
}

//
//
//
#[derive(thiserror::Error, Debug)]
pub enum EndpointError {
    #[error("MakeRequestUrlFailed {0}")]
    MakeRequestUrlFailed(UrlParseError),
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
    #[error("DeResponseBodyFailed {0}")]
    DeResponseBodyFailed(SerdeJsonError),
}

//
//
//
pub fn endpoint_parse_response<T>(response: Response<Body>) -> Result<EndpointRet<T>, EndpointError>
where
    T: fmt::Debug + Clone + DeserializeOwned,
{
    let status = response.status();
    match status {
        StatusCode::OK => {
            let ok_json = serde_json::from_slice::<T>(response.body())
                .map_err(EndpointError::DeResponseBodyFailed)?;

            Ok(EndpointRet::Ok(ok_json))
        }
        status => match serde_json::from_slice::<ResponseErrorBody>(response.body()) {
            Ok(err_json) => Ok(EndpointRet::Other((status, Ok(err_json)))),
            Err(_) => Ok(EndpointRet::Other((
                status,
                Err(response.body().to_owned()),
            ))),
        },
    }
}
