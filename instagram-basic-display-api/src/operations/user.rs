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

use super::{
    common::{
        endpoint_parse_response, EndpointError, EndpointRet, API_VERSION, BASE_URL,
        URL_PERCENT_ENCODE_ASCII_SET,
    },
    user_medias::{UserMediasResponseBody, MEDIA_FIELDS},
};
use crate::{objects::User, types::access_token::UserAccessToken};

//
#[derive(Debug, Clone)]
pub struct UserEndpoint {
    user_id: String,
    access_token: UserAccessToken,
    with_media: bool,
    //
    api_version: Option<String>,
}
impl UserEndpoint {
    pub fn new(user_id: u64, access_token: impl Into<UserAccessToken>, with_media: bool) -> Self {
        Self {
            user_id: user_id.to_string(),
            access_token: access_token.into(),
            with_media,
            api_version: None,
        }
    }

    pub fn me(access_token: impl Into<UserAccessToken>, with_media: bool) -> Self {
        Self {
            user_id: "me".to_owned(),
            access_token: access_token.into(),
            with_media,
            api_version: None,
        }
    }

    pub fn with_api_version(mut self, api_version: String) -> Self {
        self.api_version = Some(api_version);
        self
    }
}

impl Endpoint for UserEndpoint {
    type RenderRequestError = EndpointError;

    type ParseResponseOutput = EndpointRet<UserResponseBody>;
    type ParseResponseError = EndpointError;

    #[allow(clippy::vec_init_then_push)]
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
        query_pairs.push(("access_token", self.access_token.inner().to_owned()));

        let url = format!(
            "{}/{}/{}?{}",
            BASE_URL,
            self.api_version.as_deref().unwrap_or(API_VERSION),
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
            .header(USER_AGENT, "instagram-basic-display-api")
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
        assert_eq!(req.uri(), "https://graph.instagram.com/v15.0/123?fields=account_type%2Cid%2Cusername&access_token=TOKEN");

        let req = UserEndpoint::me("TOKEN".to_owned(), false)
            .render_request()
            .unwrap();
        assert_eq!(req.method(), Method::GET);
        assert_eq!(req.uri(), "https://graph.instagram.com/v15.0/me?fields=account_type%2Cid%2Cusername&access_token=TOKEN");

        let req = UserEndpoint::me("TOKEN".to_owned(), false)
            .with_api_version("v12.0".into())
            .render_request()
            .unwrap();
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
