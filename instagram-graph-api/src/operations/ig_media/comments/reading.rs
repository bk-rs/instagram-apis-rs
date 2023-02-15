use facebook_graph_api_object_paging::cursor_based_pagination::Paging;
use http_api_client_endpoint::{
    http::{
        header::{ACCEPT, USER_AGENT},
        Method, StatusCode,
    },
    Body, Endpoint, Request, Response, MIME_APPLICATION_JSON,
};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    objects::{ig_comment::IgCommentForIgMediaCommentsReadingOperation, ResponseBodyErrJson},
    operations::{
        common::{EndpointError, EndpointRet},
        URL_BASE, VERSION,
    },
};

//
#[derive(Debug, Clone)]
pub struct Reading {
    pub ig_media_id: u64,
    pub limit: Option<usize>,
    pub after: Option<Box<str>>,
    //
    pub access_token: Box<str>,
    pub version: Option<Box<str>>,
}

impl Reading {
    pub fn new(
        ig_media_id: u64,
        access_token: impl AsRef<str>,
        version: impl Into<Option<Box<str>>>,
    ) -> Self {
        Self {
            ig_media_id,
            limit: None,
            after: None,
            access_token: access_token.as_ref().into(),
            version: version.into(),
        }
    }

    pub fn limit(mut self, value: usize) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn after(mut self, value: impl AsRef<str>) -> Self {
        self.after = Some(value.as_ref().into());
        self
    }
}

impl Endpoint for Reading {
    type RenderRequestError = EndpointError;

    type ParseResponseOutput = EndpointRet<ReadingResponseBodyOkJson>;
    type ParseResponseError = EndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let url = format!(
            "{}/{}/{}/comments?fields={}",
            URL_BASE,
            self.version.as_deref().unwrap_or(VERSION),
            self.ig_media_id,
            IgCommentForIgMediaCommentsReadingOperation::fields(),
        );
        let mut url = Url::parse(&url).map_err(EndpointError::MakeRequestUrlFailed)?;

        url.query_pairs_mut()
            .append_pair("access_token", &self.access_token);
        if let Some(limit) = self.limit {
            url.query_pairs_mut()
                .append_pair("limit", limit.to_string().as_str());
        }
        if let Some(after) = &self.after {
            url.query_pairs_mut().append_pair("after", after);
        }

        let request = Request::builder()
            .method(Method::GET)
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
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ReadingResponseBodyOkJson {
    pub data: Vec<IgCommentForIgMediaCommentsReadingOperation>,
    pub paging: Option<Paging>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_response_body_ok_json() {
        //
        let content =
            include_str!("../../../../tests/response_body_json_files/v14.0/ig_media_17946328927974136__comments__reading.json");
        match serde_json::from_str::<ReadingResponseBodyOkJson>(content) {
            Ok(ok_json) => {
                // println!("{:?}", ok_json);
                assert!(!ok_json.data.is_empty());
            }
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    fn test_endpoint_render_request() {
        let ep = Reading::new(1, "ACCESS_TOKEN", None)
            .limit(30)
            .after("AFTER");
        let req = ep.render_request().unwrap();
        assert_eq!(req.method(), Method::GET);
        assert_eq!(
            req.uri().path_and_query().unwrap(),
            "/v15.0/1/comments?fields=from,hidden,id,like_count,text,timestamp,username,replies{from,hidden,id,like_count,parent_id,text,timestamp,username}&access_token=ACCESS_TOKEN&limit=30&after=AFTER"
        );
    }
}
