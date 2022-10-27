//! https://developers.facebook.com/docs/instagram-basic-display-api/reference/user/media#reading

use http_api_client_endpoint::{
    http::{
        header::{ACCEPT, USER_AGENT},
        Method,
    },
    Body, Endpoint, Request, Response,
};
use percent_encoding::percent_encode;
use serde::{Deserialize, Serialize};

use crate::objects::{Media, Paging};

use super::common::{
    endpoint_parse_response, EndpointError, EndpointRet, API_VERSION, BASE_URL,
    URL_PERCENT_ENCODE_ASCII_SET,
};

//
#[derive(Debug, Clone)]
pub struct UserMediasEndpoint {
    user_id: String,
    access_token: String,
    limit: Option<usize>,
    after: Option<String>,
    //
    api_version: Option<String>,
}
impl UserMediasEndpoint {
    pub fn new(
        user_id: u64,
        access_token: String,
        limit: Option<usize>,
        after: Option<String>,
    ) -> Self {
        Self {
            user_id: user_id.to_string(),
            access_token,
            limit,
            after,
            api_version: None,
        }
    }

    pub fn me(
        access_token: String,
        limit: impl Into<Option<usize>>,
        after: impl Into<Option<String>>,
    ) -> Self {
        Self {
            user_id: "me".to_owned(),
            access_token,
            limit: limit.into(),
            after: after.into(),
            api_version: None,
        }
    }

    pub fn with_api_version(mut self, api_version: String) -> Self {
        self.api_version = Some(api_version);
        self
    }
}

pub const MEDIA_FIELDS: &[&str] = &[
    "caption",
    "id",
    "media_type",
    "media_url",
    "permalink",
    "thumbnail_url",
    "timestamp",
    "username",
    "children.fields(id,media_type,media_url,permalink,thumbnail_url,timestamp)",
];

impl Endpoint for UserMediasEndpoint {
    type RenderRequestError = EndpointError;

    type ParseResponseOutput = EndpointRet<UserMediasResponseBody>;
    type ParseResponseError = EndpointError;

    #[allow(clippy::vec_init_then_push)]
    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let fields = MEDIA_FIELDS.join(",");

        let mut query_pairs = vec![];
        query_pairs.push((
            "fields",
            percent_encode(fields.as_bytes(), URL_PERCENT_ENCODE_ASCII_SET).to_string(),
        ));
        if let Some(limit) = &self.limit {
            query_pairs.push(("limit", format!("{}", limit)));
        }
        if let Some(after) = &self.after {
            query_pairs.push(("after", after.to_owned()));
        }
        query_pairs.push(("access_token", self.access_token.to_owned()));

        let url = format!(
            "{}/{}/{}/media?{}",
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
pub struct UserMediasResponseBody {
    pub data: Vec<Media>,
    pub paging: Option<Paging>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_request() {
        let req = UserMediasEndpoint::new(123, "TOKEN".to_owned(), None, None)
            .render_request()
            .unwrap();
        assert_eq!(req.method(), Method::GET);
        assert_eq!(req.uri(), "https://graph.instagram.com/v15.0/123/media?fields=caption%2Cid%2Cmedia_type%2Cmedia_url%2Cpermalink%2Cthumbnail_url%2Ctimestamp%2Cusername%2Cchildren.fields(id%2Cmedia_type%2Cmedia_url%2Cpermalink%2Cthumbnail_url%2Ctimestamp)&access_token=TOKEN");

        let req = UserMediasEndpoint::me("TOKEN".to_owned(), None, None)
            .render_request()
            .unwrap();
        assert_eq!(req.method(), Method::GET);
        assert_eq!(req.uri(), "https://graph.instagram.com/v15.0/me/media?fields=caption%2Cid%2Cmedia_type%2Cmedia_url%2Cpermalink%2Cthumbnail_url%2Ctimestamp%2Cusername%2Cchildren.fields(id%2Cmedia_type%2Cmedia_url%2Cpermalink%2Cthumbnail_url%2Ctimestamp)&access_token=TOKEN");

        let req = UserMediasEndpoint::me("TOKEN".to_owned(), Some(10), Some("AFTER".to_owned()))
            .render_request()
            .unwrap();
        assert_eq!(req.method(), Method::GET);
        assert_eq!(req.uri(), "https://graph.instagram.com/v15.0/me/media?fields=caption%2Cid%2Cmedia_type%2Cmedia_url%2Cpermalink%2Cthumbnail_url%2Ctimestamp%2Cusername%2Cchildren.fields(id%2Cmedia_type%2Cmedia_url%2Cpermalink%2Cthumbnail_url%2Ctimestamp)&limit=10&after=AFTER&access_token=TOKEN");
    }

    #[test]
    fn test_de_response_body() {
        let body = serde_json::from_str::<UserMediasResponseBody>(include_str!(
            "../../tests/response_body_files/user_medias_ok.json"
        ))
        .unwrap();
        assert_eq!(body.data.len(), 36);

        let body = serde_json::from_str::<UserMediasResponseBody>(include_str!(
            "../../tests/response_body_files/user_medias_ok__limit_1.json"
        ))
        .unwrap();
        assert_eq!(body.data.len(), 1);
        assert!(body.paging.unwrap().next_cursor().is_some());
    }
}
