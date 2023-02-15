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
pub struct UpdatingWithEnableOrDisableComments {
    pub ig_media_id: u64,
    pub comment_enabled: bool,
    //
    pub access_token: Box<str>,
    pub version: Option<Box<str>>,
}

impl UpdatingWithEnableOrDisableComments {
    pub fn new(
        ig_media_id: u64,
        comment_enabled: bool,
        access_token: impl AsRef<str>,
        version: impl Into<Option<Box<str>>>,
    ) -> Self {
        Self {
            ig_media_id,
            comment_enabled,
            access_token: access_token.as_ref().into(),
            version: version.into(),
        }
    }
}

impl Endpoint for UpdatingWithEnableOrDisableComments {
    type RenderRequestError = EndpointError;

    type ParseResponseOutput = EndpointRet<UpdatingWithEnableOrDisableCommentsResponseBodyOkJson>;
    type ParseResponseError = EndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let url = format!(
            "{}/{}/{}",
            URL_BASE,
            self.version.as_deref().unwrap_or(VERSION),
            self.ig_media_id,
        );
        let mut url = Url::parse(&url).map_err(EndpointError::MakeRequestUrlFailed)?;

        url.query_pairs_mut()
            .append_pair("comment_enabled", self.comment_enabled.to_string().as_str());
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
pub struct UpdatingWithEnableOrDisableCommentsResponseBodyOkJson {
    pub success: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_response_body_ok_json() {
        //
        let content = include_str!(
            "../../../tests/response_body_json_files/v14.0/ig_media_0__updating__comment_enabled_sample.json"
        );
        match serde_json::from_str::<UpdatingWithEnableOrDisableCommentsResponseBodyOkJson>(content)
        {
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
        let ep = UpdatingWithEnableOrDisableComments::new(1, true, "ACCESS_TOKEN", None);
        let req = ep.render_request().unwrap();
        assert_eq!(req.method(), Method::POST);
        assert_eq!(
            req.uri().path_and_query().unwrap(),
            "/v15.0/1?comment_enabled=true&access_token=ACCESS_TOKEN"
        );

        //
        let ep = UpdatingWithEnableOrDisableComments::new(1, false, "ACCESS_TOKEN", None);
        let req = ep.render_request().unwrap();
        assert_eq!(req.method(), Method::POST);
        assert_eq!(
            req.uri().path_and_query().unwrap(),
            "/v15.0/1?comment_enabled=false&access_token=ACCESS_TOKEN"
        );
    }
}
