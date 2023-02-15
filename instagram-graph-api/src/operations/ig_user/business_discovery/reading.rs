use chrono::{DateTime, Utc};
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
    objects::{IgUserForIgUserBusinessDiscoveryReadingOperation, ResponseBodyErrJson},
    operations::{
        common::{EndpointError, EndpointRet},
        URL_BASE, VERSION,
    },
};

//
#[derive(Debug, Clone)]
pub struct Reading {
    pub ig_user_id: u64,
    pub username: Box<str>,
    pub media_limit: Option<usize>,
    pub media_since: Option<DateTime<Utc>>,
    //
    pub access_token: Box<str>,
    pub version: Option<Box<str>>,
}

impl Reading {
    pub fn new(
        ig_user_id: u64,
        username: impl AsRef<str>,
        access_token: impl AsRef<str>,
        version: impl Into<Option<Box<str>>>,
    ) -> Self {
        Self {
            ig_user_id,
            username: username.as_ref().into(),
            media_limit: None,
            media_since: None,
            access_token: access_token.as_ref().into(),
            version: version.into(),
        }
    }

    pub fn media_limit(mut self, value: usize) -> Self {
        self.media_limit = Some(value);
        self
    }

    pub fn media_since(mut self, value: DateTime<Utc>) -> Self {
        self.media_since = Some(value);
        self
    }
}

impl Endpoint for Reading {
    type RenderRequestError = EndpointError;

    type ParseResponseOutput = EndpointRet<ReadingResponseBodyRet>;
    type ParseResponseError = EndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let url = format!(
            "{}/{}/{}?fields=business_discovery.username({}){{{}}}",
            URL_BASE,
            self.version.as_deref().unwrap_or(VERSION),
            self.ig_user_id,
            &self.username,
            IgUserForIgUserBusinessDiscoveryReadingOperation::fields(
                self.media_since,
                self.media_limit
            ),
        );
        let mut url = Url::parse(&url).map_err(EndpointError::MakeRequestUrlFailed)?;

        url.query_pairs_mut()
            .append_pair("access_token", &self.access_token);

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
            StatusCode::OK => Ok(EndpointRet::Ok(ReadingResponseBodyRet::OkJson(
                serde_json::from_slice(response.body())
                    .map_err(EndpointError::DeResponseBodyOkJsonFailed)?,
            ))),
            status => match serde_json::from_slice::<ResponseBodyErrJson>(response.body()) {
                Ok(err_json) => {
                    if err_json.is_ig_user_business_discovery_cannot_find_user() {
                        Ok(EndpointRet::Ok(ReadingResponseBodyRet::CannotFindUser))
                    } else {
                        Ok(EndpointRet::Other((status, Ok(err_json))))
                    }
                }
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
    pub business_discovery: IgUserForIgUserBusinessDiscoveryReadingOperation,
}

#[derive(Debug, Clone)]
pub enum ReadingResponseBodyRet {
    OkJson(Box<ReadingResponseBodyOkJson>),
    CannotFindUser,
}

impl ReadingResponseBodyRet {
    pub fn as_ok_json(&self) -> Option<&ReadingResponseBodyOkJson> {
        match self {
            ReadingResponseBodyRet::OkJson(x) => Some(x),
            ReadingResponseBodyRet::CannotFindUser => None,
        }
    }
}

//
impl ResponseBodyErrJson {
    pub fn is_ig_user_business_discovery_cannot_find_user(&self) -> bool {
        self.error
            .error_user_title
            .as_ref()
            .map(|x| x.to_lowercase().contains("cannot find User"))
            == Some(true)
            || self.error.error_subcode == Some(2207013)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_response_body_ok_json() {
        //
        let content =
            include_str!("../../../../tests/response_body_json_files/v14.0/ig_user_0__business_discovery__reading__username_bluebottle.json");
        match serde_json::from_str::<ReadingResponseBodyOkJson>(content) {
            Ok(ok_json) => {
                // println!("{:?}", ok_json);
                assert_eq!(ok_json.business_discovery.username, "bluebottle");
            }
            Err(err) => panic!("{}", err),
        }

        //
        let content =
            include_str!("../../../../tests/response_body_json_files/v14.0/ig_user_0__business_discovery__reading__username_qq122755990.json");
        match serde_json::from_str::<ReadingResponseBodyOkJson>(content) {
            Ok(ok_json) => {
                // println!("{:?}", ok_json);
                assert_eq!(ok_json.business_discovery.username, "qq122755990");
            }
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    fn test_de_response_body_err_json() {
        //
        let content = include_str!("../../../../tests/response_body_json_files/v14.0/err__ig_user_0__business_discovery__reading__not_business.json");
        match serde_json::from_str::<ResponseBodyErrJson>(content) {
            Ok(err_json) => {
                assert!(err_json.is_ig_user_business_discovery_cannot_find_user())
            }
            Err(err) => panic!("{}", err),
        }

        //
        let content = include_str!("../../../../tests/response_body_json_files/v14.0/err__ig_user_0__business_discovery__reading__not_exists.json");
        match serde_json::from_str::<ResponseBodyErrJson>(content) {
            Ok(err_json) => {
                assert!(err_json.is_ig_user_business_discovery_cannot_find_user())
            }
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    fn test_endpoint_render_request() {
        let ep = Reading::new(1, "foo", "ACCESS_TOKEN", None)
            .media_limit(30)
            .media_since("2022-01-01T00:00:00Z".parse().unwrap());
        let req = ep.render_request().unwrap();
        assert_eq!(req.method(), Method::GET);
        assert_eq!(
            req.uri().path_and_query().unwrap(),
            "/v15.0/1?fields=business_discovery.username(foo){biography,id,ig_id,followers_count,follows_count,media_count,name,profile_picture_url,username,website,media.since(1640995200).limit(30){caption,comments_count,id,like_count,media_product_type,media_type,media_url,permalink,timestamp,children{id,media_type,media_url,permalink,timestamp}}}&access_token=ACCESS_TOKEN"
        );
    }
}
