use http_api_client_endpoint::{
    http::{
        header::{ACCEPT, USER_AGENT},
        Method, StatusCode,
    },
    Body, Endpoint, Request, Response, MIME_APPLICATION_JSON,
};
use url::Url;

use crate::{
    objects::{ig_comment::IgCommentForIgMediaCommentsCreatingOperation, ResponseBodyErrJson},
    operations::{
        common::{EndpointError, EndpointRet},
        URL_BASE, VERSION,
    },
};

//
#[derive(Debug, Clone)]
pub struct Creating {
    pub ig_media_id: u64,
    pub message: Box<str>,
    //
    pub access_token: Box<str>,
    pub version: Option<Box<str>>,
}

impl Creating {
    pub fn new(
        ig_media_id: u64,
        message: impl AsRef<str>,
        access_token: impl AsRef<str>,
        version: impl Into<Option<Box<str>>>,
    ) -> Self {
        Self {
            ig_media_id,
            message: message.as_ref().into(),
            access_token: access_token.as_ref().into(),
            version: version.into(),
        }
    }
}

impl Endpoint for Creating {
    type RenderRequestError = EndpointError;

    type ParseResponseOutput = EndpointRet<CreatingResponseBodyOkJson>;
    type ParseResponseError = EndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let url = format!(
            "{}/{}/{}/comments?fields={}",
            URL_BASE,
            self.version.as_deref().unwrap_or(VERSION),
            self.ig_media_id,
            IgCommentForIgMediaCommentsCreatingOperation::fields(),
        );
        let mut url = Url::parse(&url).map_err(EndpointError::MakeRequestUrlFailed)?;

        url.query_pairs_mut().append_pair("message", &self.message);
        url.query_pairs_mut()
            .append_pair("access_token", &self.access_token);

        let request = Request::builder()
            .method(Method::POST)
            .uri(url.as_str())
            .header(USER_AGENT, "instagram-graph-api")
            .header(ACCEPT, MIME_APPLICATION_JSON)
            .body(vec![])
            .map_err(EndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        let status = response.status();
        match status {
            StatusCode::OK => Ok(EndpointRet::Ok(
                serde_json::from_slice(response.body())
                    .map_err(EndpointError::DeResponseBodyOkJsonFailed)?,
            )),
            status => match serde_json::from_slice::<ResponseBodyErrJson>(response.body()) {
                Ok(err_json) => Ok(EndpointRet::Other((status, Ok(err_json)))),
                Err(_) => Ok(EndpointRet::Other((
                    status,
                    Err(response.body().to_owned()),
                ))),
            },
        }
    }
}

//
//
//
pub type CreatingResponseBodyOkJson = IgCommentForIgMediaCommentsCreatingOperation;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_response_body_ok_json() {
        //
        let content =
            include_str!("../../../../tests/response_body_json_files/v14.0/ig_media_17946328927974136__comments__creating.json");
        match serde_json::from_str::<CreatingResponseBodyOkJson>(content) {
            Ok(ok_json) => {
                // println!("{:?}", ok_json);
                assert!(ok_json.id > 0);
            }
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    fn test_endpoint_render_request() {
        let ep = Creating::new(1, "foo", "ACCESS_TOKEN", None);
        let req = ep.render_request().unwrap();
        assert_eq!(req.method(), Method::POST);
        assert_eq!(
            req.uri().path_and_query().unwrap(),
            "/v15.0/1/comments?fields=from,hidden,id,like_count,text,timestamp,username&message=foo&access_token=ACCESS_TOKEN"
        );
    }
}
