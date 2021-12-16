//! https://developers.facebook.com/docs/instagram-basic-display-api/reference/access_token#reading

use http_api_client_endpoint::{
    http::{
        header::{ACCEPT, USER_AGENT},
        Method,
    },
    Body, Endpoint, Request, Response,
};

use serde::{Deserialize, Serialize};
use url::Url;

use super::common::{endpoint_parse_response, EndpointError, EndpointRet};

pub const URL: &str = "https://graph.instagram.com/access_token";

pub struct ExchangeSlAccessTokenForLlAccessTokenEndpoint {
    client_secret: String,
    access_token: String,
}
impl ExchangeSlAccessTokenForLlAccessTokenEndpoint {
    pub fn new(client_secret: String, access_token: String) -> Self {
        Self {
            client_secret,
            access_token,
        }
    }
}

impl Endpoint for ExchangeSlAccessTokenForLlAccessTokenEndpoint {
    type RenderRequestError = EndpointError;

    type ParseResponseOutput = EndpointRet<ExchangeSlAccessTokenForLlAccessTokenResponseBody>;
    type ParseResponseError = EndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let mut url = Url::parse(URL).map_err(EndpointError::MakeRequestUrlFailed)?;

        url.query_pairs_mut()
            .append_pair("grant_type", "ig_exchange_token")
            .append_pair("client_secret", &self.client_secret)
            .append_pair("access_token", &self.access_token);

        let request = Request::builder()
            .method(Method::GET)
            .uri(url.as_str())
            .header(USER_AGENT, "curl/7.72.0")
            .header(ACCEPT, "application/json")
            .body(vec![])
            .map_err(EndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        endpoint_parse_response(response)
    }
}

//
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExchangeSlAccessTokenForLlAccessTokenResponseBody {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_request() {
        let req = ExchangeSlAccessTokenForLlAccessTokenEndpoint::new(
            "SECRET".to_owned(),
            "TOKEN".to_owned(),
        )
        .render_request()
        .unwrap();
        assert_eq!(req.method(), Method::GET);
        assert_eq!(req.uri(), "https://graph.instagram.com/access_token?grant_type=ig_exchange_token&client_secret=SECRET&access_token=TOKEN");
    }

    #[test]
    fn test_de_response_body() {
        let body = serde_json::from_str::<ExchangeSlAccessTokenForLlAccessTokenResponseBody>(
            include_str!("../../tests/response_body_files/exchange_sl_access_token_for_ll_access_token_ok.json"),
        )
        .unwrap();

        assert_eq!(body.expires_in, Some(5184000));
    }
}
