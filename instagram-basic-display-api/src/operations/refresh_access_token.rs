//! https://developers.facebook.com/docs/instagram-basic-display-api/reference/refresh_access_token#reading

use http_api_client_endpoint::{
    http::{
        header::{ACCEPT, USER_AGENT},
        Method,
    },
    Body, Endpoint, Request, Response,
};
use url::Url;

use super::{
    common::{endpoint_parse_response, EndpointError, EndpointRet},
    ExchangeSlAccessTokenForLlAccessTokenResponseBody,
};

pub const URL: &str = "https://graph.instagram.com/refresh_access_token";

pub struct RefreshAccessTokenEndpoint {
    access_token: String,
}
impl RefreshAccessTokenEndpoint {
    pub fn new(access_token: String) -> Self {
        Self { access_token }
    }
}

pub type RefreshAccessTokenResponseBody = ExchangeSlAccessTokenForLlAccessTokenResponseBody;

impl Endpoint for RefreshAccessTokenEndpoint {
    type RenderRequestError = EndpointError;

    type ParseResponseOutput = EndpointRet<RefreshAccessTokenResponseBody>;
    type ParseResponseError = EndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let mut url = Url::parse(URL).map_err(EndpointError::MakeRequestUrlFailed)?;

        url.query_pairs_mut()
            .append_pair("grant_type", "ig_refresh_token")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_request() {
        let req = RefreshAccessTokenEndpoint::new("TOKEN".to_owned())
            .render_request()
            .unwrap();
        assert_eq!(req.method(), Method::GET);
        assert_eq!(req.uri(), "https://graph.instagram.com/refresh_access_token?grant_type=ig_refresh_token&access_token=TOKEN");
    }

    #[test]
    fn test_de_response_body() {
        let body = serde_json::from_str::<RefreshAccessTokenResponseBody>(include_str!(
            "../../tests/response_body_files/refresh_access_token_ok.json"
        ))
        .unwrap();

        assert_eq!(body.expires_in, Some(5183828));
    }
}
