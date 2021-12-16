//! https://developers.facebook.com/docs/instagram-basic-display-api/reference/user#reading

use http_api_client_endpoint::{
    http::{
        header::{ACCEPT, USER_AGENT},
        Method,
    },
    Body, Endpoint, Request, Response,
};
use percent_encoding::percent_encode;
use serde::{Deserialize, Serialize};

use crate::objects::User;

use super::{
    common::{
        endpoint_parse_response, EndpointError, EndpointRet, URL_PERCENT_ENCODE_ASCII_SET,
        URL_PREFIX,
    },
    user_medias::{UserMediasResponseBody, MEDIA_FIELDS},
};

pub struct UserEndpoint {
    user_id: String,
    access_token: String,
    with_media: bool,
}
impl UserEndpoint {
    pub fn new(user_id: u64, access_token: String, with_media: bool) -> Self {
        Self {
            user_id: user_id.to_string(),
            access_token,
            with_media,
        }
    }

    pub fn me(access_token: String, with_media: bool) -> Self {
        Self {
            user_id: "me".to_owned(),
            access_token,
            with_media,
        }
    }
}

impl Endpoint for UserEndpoint {
    type RenderRequestError = EndpointError;

    type ParseResponseOutput = EndpointRet<UserResponseBody>;
    type ParseResponseError = EndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let media_fields = format!("media.fields({})", MEDIA_FIELDS.join(","));

        let mut fields = vec!["account_type", "id", "username"];
        if self.with_media {
            fields.push("media_count");
            fields.push(media_fields.as_str());
        }
        let fields = fields.join(",");

        let mut query_pairs = vec![];
        query_pairs.push((
            "fields",
            percent_encode(fields.as_bytes(), URL_PERCENT_ENCODE_ASCII_SET).to_string(),
        ));
        query_pairs.push(("access_token", self.access_token.to_owned()));

        let url = format!(
            "{}/{}?{}",
            URL_PREFIX,
            self.user_id,
            query_pairs
                .into_iter()
                .map(|x| format!("{}={}", x.0, x.1))
                .collect::<Vec<String>>()
                .join("&"),
        );

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
pub struct UserResponseBody {
    #[serde(flatten)]
    pub basic: User,
    #[serde(default)]
    pub media: Option<UserMediasResponseBody>,
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::objects::AccountType;

    #[test]
    fn test_render_request() {
        let req = UserEndpoint::new(123, "TOKEN".to_owned(), false)
            .render_request()
            .unwrap();
        assert_eq!(req.method(), Method::GET);
        assert_eq!(req.uri(), "https://graph.instagram.com/v12.0/123?fields=account_type%2Cid%2Cusername&access_token=TOKEN");

        let req = UserEndpoint::me("TOKEN".to_owned(), false)
            .render_request()
            .unwrap();
        assert_eq!(req.method(), Method::GET);
        assert_eq!(req.uri(), "https://graph.instagram.com/v12.0/me?fields=account_type%2Cid%2Cusername&access_token=TOKEN");
    }

    #[test]
    fn test_de_response_body() {
        let body = serde_json::from_str::<UserResponseBody>(include_str!(
            "../../tests/response_body_files/me_with_media_ok.json"
        ))
        .unwrap();

        assert_eq!(body.basic.account_type, AccountType::Business);
        assert_eq!(body.basic.id, 6489782497758472);
        assert_eq!(body.media.unwrap().data.len(), 11);

        //
        let body = serde_json::from_str::<UserResponseBody>(include_str!(
            "../../tests/response_body_files/me_without_media_ok.json"
        ))
        .unwrap();

        assert_eq!(body.basic.account_type, AccountType::Business);
        assert_eq!(body.basic.id, 6489782497758472);
        assert!(body.basic.media_count.is_none());
        assert!(body.media.is_none());
    }
}
