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
    objects::ResponseBodyErrJson,
    operations::{
        common::{EndpointError, EndpointRet},
        URL_BASE, VERSION,
    },
};

//
#[derive(Debug, Clone)]
pub struct UpdatingWithHideOrUnhide {
    pub ig_comment_id: u64,
    pub hide: bool,
    //
    pub access_token: Box<str>,
    pub version: Option<Box<str>>,
}

impl UpdatingWithHideOrUnhide {
    pub fn new(
        ig_comment_id: u64,
        hide: bool,
        access_token: impl AsRef<str>,
        version: impl Into<Option<Box<str>>>,
    ) -> Self {
        Self {
            ig_comment_id,
            hide,
            access_token: access_token.as_ref().into(),
            version: version.into(),
        }
    }
}

impl Endpoint for UpdatingWithHideOrUnhide {
    type RenderRequestError = EndpointError;

    type ParseResponseOutput = EndpointRet<UpdatingWithHideOrUnhideResponseBodyOkJson>;
    type ParseResponseError = EndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let url = format!(
            "{}/{}/{}",
            URL_BASE,
            self.version.as_deref().unwrap_or(VERSION),
            self.ig_comment_id,
        );
        let mut url = Url::parse(&url).map_err(EndpointError::MakeRequestUrlFailed)?;

        url.query_pairs_mut()
            .append_pair("hide", self.hide.to_string().as_str());
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
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdatingWithHideOrUnhideResponseBodyOkJson {
    pub success: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_response_body_ok_json() {
        //
        let content = include_str!(
            "../../../tests/response_body_json_files/v14.0/ig_comment_0__updating__hide_sample.json"
        );
        match serde_json::from_str::<UpdatingWithHideOrUnhideResponseBodyOkJson>(content) {
            Ok(ok_json) => {
                // println!("{:?}", ok_json);
                assert!(ok_json.success);
            }
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    fn test_endpoint_render_request() {
        //
        let ep = UpdatingWithHideOrUnhide::new(1, true, "ACCESS_TOKEN", None);
        let req = ep.render_request().unwrap();
        assert_eq!(req.method(), Method::POST);
        assert_eq!(
            req.uri().path_and_query().unwrap(),
            "/v15.0/1?hide=true&access_token=ACCESS_TOKEN"
        );

        //
        let ep = UpdatingWithHideOrUnhide::new(1, false, "ACCESS_TOKEN", None);
        let req = ep.render_request().unwrap();
        assert_eq!(req.method(), Method::POST);
        assert_eq!(
            req.uri().path_and_query().unwrap(),
            "/v15.0/1?hide=false&access_token=ACCESS_TOKEN"
        );
    }
}
