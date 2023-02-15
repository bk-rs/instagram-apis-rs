use core::ops::Deref;

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
    objects::{IgContainer, ResponseBodyErrJson},
    operations::{
        common::{EndpointError, EndpointRet},
        URL_BASE, VERSION,
    },
};

//
#[derive(Debug, Clone)]
pub struct CreatingWithImage {
    pub ig_user_id: u64,
    pub image_url: Box<str>,
    pub caption: Option<Box<str>>,
    // Require page.location.latitude present and page.location.longitude present
    pub location_id: Option<u64>,
    pub user_tags: Option<Vec<ValueUserTag>>,
    //
    pub access_token: Box<str>,
    pub version: Option<Box<str>>,
}

impl CreatingWithImage {
    pub fn new(
        ig_user_id: u64,
        image_url: impl AsRef<str>,
        access_token: impl AsRef<str>,
        version: impl Into<Option<Box<str>>>,
    ) -> Self {
        Self {
            ig_user_id,
            image_url: image_url.as_ref().into(),
            caption: None,
            location_id: None,
            user_tags: None,
            access_token: access_token.as_ref().into(),
            version: version.into(),
        }
    }

    pub fn caption(mut self, value: impl AsRef<str>) -> Self {
        self.caption = Some(value.as_ref().into());
        self
    }

    pub fn location_id(mut self, value: u64) -> Self {
        self.location_id = Some(value);
        self
    }

    pub fn user_tags(mut self, value: Vec<ValueUserTag>) -> Self {
        self.user_tags = Some(value);
        self
    }
}

impl Endpoint for CreatingWithImage {
    type RenderRequestError = EndpointError;

    type ParseResponseOutput = EndpointRet<CreatingResponseBodyRet>;
    type ParseResponseError = EndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let url = format!(
            "{}/{}/{}/media?fields={}",
            URL_BASE,
            self.version.as_deref().unwrap_or(VERSION),
            self.ig_user_id,
            IgContainer::fields(),
        );
        let mut url = Url::parse(&url).map_err(EndpointError::MakeRequestUrlFailed)?;

        url.query_pairs_mut()
            .append_pair("image_url", &self.image_url);
        url.query_pairs_mut()
            .append_pair("is_carousel_item", false.to_string().as_ref());

        if let Some(caption) = &self.caption {
            url.query_pairs_mut().append_pair("caption", caption);
        }
        if let Some(location_id) = &self.location_id {
            url.query_pairs_mut()
                .append_pair("location_id", location_id.to_string().as_ref());
        }
        if let Some(user_tags) = &self.user_tags {
            url.query_pairs_mut().append_pair(
                "user_tags",
                serde_json::to_string(&user_tags)
                    .map_err(|_| EndpointError::Other("ser user_tags failed".into()))?
                    .as_ref(),
            );
        }

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
        parse_response(response)
    }
}

//
#[derive(Debug, Clone)]
pub struct CreatingWithCarouselItemImage {
    pub ig_user_id: u64,
    pub image_url: Box<str>,
    pub user_tags: Option<Vec<ValueUserTag>>,
    //
    pub access_token: Box<str>,
    pub version: Option<Box<str>>,
}

impl CreatingWithCarouselItemImage {
    pub fn new(
        ig_user_id: u64,
        image_url: impl AsRef<str>,
        access_token: impl AsRef<str>,
        version: impl Into<Option<Box<str>>>,
    ) -> Self {
        Self {
            ig_user_id,
            image_url: image_url.as_ref().into(),
            user_tags: None,
            access_token: access_token.as_ref().into(),
            version: version.into(),
        }
    }

    pub fn user_tags(mut self, value: Vec<ValueUserTag>) -> Self {
        self.user_tags = Some(value);
        self
    }
}

impl Endpoint for CreatingWithCarouselItemImage {
    type RenderRequestError = EndpointError;

    type ParseResponseOutput = EndpointRet<CreatingResponseBodyRet>;
    type ParseResponseError = EndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let url = format!(
            "{}/{}/{}/media?fields={}",
            URL_BASE,
            self.version.as_deref().unwrap_or(VERSION),
            self.ig_user_id,
            IgContainer::fields(),
        );
        let mut url = Url::parse(&url).map_err(EndpointError::MakeRequestUrlFailed)?;

        url.query_pairs_mut()
            .append_pair("image_url", &self.image_url);
        url.query_pairs_mut()
            .append_pair("is_carousel_item", true.to_string().as_ref());

        if let Some(user_tags) = &self.user_tags {
            url.query_pairs_mut().append_pair(
                "user_tags",
                serde_json::to_string(&user_tags)
                    .map_err(|_| EndpointError::Other("ser user_tags failed".into()))?
                    .as_ref(),
            );
        }

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
        parse_response(response)
    }
}

//
#[derive(Debug, Clone)]
pub struct CreatingWithVideo {
    pub ig_user_id: u64,
    pub video_url: Box<str>,
    pub caption: Option<Box<str>>,
    pub location_id: Option<u64>,
    pub thumb_offset: Option<u64>,
    //
    pub access_token: Box<str>,
    pub version: Option<Box<str>>,
}

impl CreatingWithVideo {
    pub fn new(
        ig_user_id: u64,
        video_url: impl AsRef<str>,
        access_token: impl AsRef<str>,
        version: impl Into<Option<Box<str>>>,
    ) -> Self {
        Self {
            ig_user_id,
            video_url: video_url.as_ref().into(),
            caption: None,
            location_id: None,
            thumb_offset: None,
            access_token: access_token.as_ref().into(),
            version: version.into(),
        }
    }

    pub fn caption(mut self, value: impl AsRef<str>) -> Self {
        self.caption = Some(value.as_ref().into());
        self
    }

    pub fn location_id(mut self, value: u64) -> Self {
        self.location_id = Some(value);
        self
    }

    pub fn thumb_offset(mut self, value: u64) -> Self {
        self.thumb_offset = Some(value);
        self
    }
}

impl Endpoint for CreatingWithVideo {
    type RenderRequestError = EndpointError;

    type ParseResponseOutput = EndpointRet<CreatingResponseBodyRet>;
    type ParseResponseError = EndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let url = format!(
            "{}/{}/{}/media?fields={}",
            URL_BASE,
            self.version.as_deref().unwrap_or(VERSION),
            self.ig_user_id,
            IgContainer::fields(),
        );
        let mut url = Url::parse(&url).map_err(EndpointError::MakeRequestUrlFailed)?;

        url.query_pairs_mut().append_pair("media_type", "VIDEO");
        url.query_pairs_mut()
            .append_pair("video_url", &self.video_url);
        url.query_pairs_mut()
            .append_pair("is_carousel_item", false.to_string().as_ref());

        if let Some(caption) = &self.caption {
            url.query_pairs_mut().append_pair("caption", caption);
        }
        if let Some(location_id) = &self.location_id {
            url.query_pairs_mut()
                .append_pair("location_id", location_id.to_string().as_ref());
        }
        if let Some(thumb_offset) = &self.thumb_offset {
            url.query_pairs_mut()
                .append_pair("thumb_offset", thumb_offset.to_string().as_ref());
        }

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
        parse_response(response)
    }
}

//
#[derive(Debug, Clone)]
pub struct CreatingWithCarouselItemVideo {
    pub ig_user_id: u64,
    pub video_url: Box<str>,
    pub thumb_offset: Option<u64>,
    //
    pub access_token: Box<str>,
    pub version: Option<Box<str>>,
}

impl CreatingWithCarouselItemVideo {
    pub fn new(
        ig_user_id: u64,
        video_url: impl AsRef<str>,
        access_token: impl AsRef<str>,
        version: impl Into<Option<Box<str>>>,
    ) -> Self {
        Self {
            ig_user_id,
            video_url: video_url.as_ref().into(),
            thumb_offset: None,
            access_token: access_token.as_ref().into(),
            version: version.into(),
        }
    }

    pub fn thumb_offset(mut self, value: u64) -> Self {
        self.thumb_offset = Some(value);
        self
    }
}

impl Endpoint for CreatingWithCarouselItemVideo {
    type RenderRequestError = EndpointError;

    type ParseResponseOutput = EndpointRet<CreatingResponseBodyRet>;
    type ParseResponseError = EndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let url = format!(
            "{}/{}/{}/media?fields={}",
            URL_BASE,
            self.version.as_deref().unwrap_or(VERSION),
            self.ig_user_id,
            IgContainer::fields(),
        );
        let mut url = Url::parse(&url).map_err(EndpointError::MakeRequestUrlFailed)?;

        url.query_pairs_mut().append_pair("media_type", "VIDEO");
        url.query_pairs_mut()
            .append_pair("video_url", &self.video_url);
        url.query_pairs_mut()
            .append_pair("is_carousel_item", true.to_string().as_ref());

        if let Some(thumb_offset) = &self.thumb_offset {
            url.query_pairs_mut()
                .append_pair("thumb_offset", thumb_offset.to_string().as_ref());
        }

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
        parse_response(response)
    }
}

//
#[derive(Debug, Clone)]
pub struct CreatingWithCarousel {
    pub ig_user_id: u64,
    pub children: Vec<u64>,
    pub caption: Option<Box<str>>,
    pub location_id: Option<u64>,
    //
    pub access_token: Box<str>,
    pub version: Option<Box<str>>,
}

impl CreatingWithCarousel {
    pub fn new(
        ig_user_id: u64,
        children: Vec<u64>,
        access_token: impl AsRef<str>,
        version: impl Into<Option<Box<str>>>,
    ) -> Self {
        Self {
            ig_user_id,
            children,
            caption: None,
            location_id: None,
            access_token: access_token.as_ref().into(),
            version: version.into(),
        }
    }

    pub fn caption(mut self, value: impl AsRef<str>) -> Self {
        self.caption = Some(value.as_ref().into());
        self
    }

    pub fn location_id(mut self, value: u64) -> Self {
        self.location_id = Some(value);
        self
    }
}

impl Endpoint for CreatingWithCarousel {
    type RenderRequestError = EndpointError;

    type ParseResponseOutput = EndpointRet<CreatingResponseBodyRet>;
    type ParseResponseError = EndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let url = format!(
            "{}/{}/{}/media?fields={}",
            URL_BASE,
            self.version.as_deref().unwrap_or(VERSION),
            self.ig_user_id,
            IgContainer::fields(),
        );
        let mut url = Url::parse(&url).map_err(EndpointError::MakeRequestUrlFailed)?;

        url.query_pairs_mut().append_pair("media_type", "CAROUSEL");
        url.query_pairs_mut().append_pair(
            "children",
            serde_json::to_string(&self.children)
                .map_err(|_| EndpointError::Other("ser children failed".into()))?
                .as_ref(),
        );

        if let Some(caption) = &self.caption {
            url.query_pairs_mut().append_pair("caption", caption);
        }
        if let Some(location_id) = &self.location_id {
            url.query_pairs_mut()
                .append_pair("location_id", location_id.to_string().as_ref());
        }

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
        parse_response(response)
    }
}

//
#[derive(Debug, Clone)]
pub struct CreatingWithReels {
    pub ig_user_id: u64,
    pub video_url: Box<str>,
    pub caption: Option<Box<str>>,
    pub location_id: Option<u64>,
    pub thumb_offset: Option<u64>,
    pub share_to_feed: Option<bool>,
    //
    pub access_token: Box<str>,
    pub version: Option<Box<str>>,
}

impl CreatingWithReels {
    pub fn new(
        ig_user_id: u64,
        video_url: impl AsRef<str>,
        access_token: impl AsRef<str>,
        version: impl Into<Option<Box<str>>>,
    ) -> Self {
        Self {
            ig_user_id,
            video_url: video_url.as_ref().into(),
            caption: None,
            location_id: None,
            thumb_offset: None,
            share_to_feed: None,
            access_token: access_token.as_ref().into(),
            version: version.into(),
        }
    }

    pub fn caption(mut self, value: impl AsRef<str>) -> Self {
        self.caption = Some(value.as_ref().into());
        self
    }

    pub fn location_id(mut self, value: u64) -> Self {
        self.location_id = Some(value);
        self
    }

    pub fn thumb_offset(mut self, value: u64) -> Self {
        self.thumb_offset = Some(value);
        self
    }

    pub fn share_to_feed(mut self, value: bool) -> Self {
        self.share_to_feed = Some(value);
        self
    }
}

impl Endpoint for CreatingWithReels {
    type RenderRequestError = EndpointError;

    type ParseResponseOutput = EndpointRet<CreatingResponseBodyRet>;
    type ParseResponseError = EndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let url = format!(
            "{}/{}/{}/media?fields={}",
            URL_BASE,
            self.version.as_deref().unwrap_or(VERSION),
            self.ig_user_id,
            IgContainer::fields(),
        );
        let mut url = Url::parse(&url).map_err(EndpointError::MakeRequestUrlFailed)?;

        url.query_pairs_mut().append_pair("media_type", "REELS");
        url.query_pairs_mut()
            .append_pair("video_url", &self.video_url);

        if let Some(caption) = &self.caption {
            url.query_pairs_mut().append_pair("caption", caption);
        }
        if let Some(location_id) = &self.location_id {
            url.query_pairs_mut()
                .append_pair("location_id", location_id.to_string().as_ref());
        }
        if let Some(thumb_offset) = &self.thumb_offset {
            url.query_pairs_mut()
                .append_pair("thumb_offset", thumb_offset.to_string().as_ref());
        }
        if let Some(share_to_feed) = &self.share_to_feed {
            url.query_pairs_mut()
                .append_pair("share_to_feed", share_to_feed.to_string().as_ref());
        }

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
        parse_response(response)
    }
}

//
//
//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreatingResponseBodyOkJson(pub IgContainer);

impl Deref for CreatingResponseBodyOkJson {
    type Target = IgContainer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub enum CreatingResponseBodyRet {
    OkJson(CreatingResponseBodyOkJson),
    ExtInfoError(ExtInfoError),
}

error_macro::r#enum! {
    #[derive(Clone)]
    pub enum ExtInfoError {
        ImageUrlOrVideoUrlInvalid(Box<str>),
        UserTagUsernamePrivateOrInvalid(Box<str>),
        ImageAspectRatioInvalid(Box<str>),
        AccountNotAllowedCreate(Box<str>),
    }
}

impl ExtInfoError {
    pub fn from_err_json(err_json: &ResponseBodyErrJson) -> Option<Self> {
        match (err_json.error.code, err_json.error.error_subcode) {
            (9004, Some(2207052)) => Some(Self::ImageUrlOrVideoUrlInvalid(
                err_json
                    .error
                    .error_user_title
                    .as_deref()
                    .unwrap_or_else(|| err_json.error.message.as_ref())
                    .into(),
            )),
            (110, Some(2207018)) => Some(Self::UserTagUsernamePrivateOrInvalid(
                err_json
                    .error
                    .error_user_msg
                    .as_deref()
                    .unwrap_or_else(|| err_json.error.message.as_ref())
                    .into(),
            )),
            (36003, Some(2207009)) => Some(Self::ImageAspectRatioInvalid(
                err_json
                    .error
                    .error_user_msg
                    .as_deref()
                    .unwrap_or_else(|| err_json.error.message.as_ref())
                    .into(),
            )),
            _ => {
                if err_json
                    .error
                    .message
                    .to_ascii_lowercase()
                    .contains("account is not allowed to create")
                {
                    Some(Self::AccountNotAllowedCreate(
                        err_json.error.message.to_owned().into(),
                    ))
                } else {
                    None
                }
            }
        }
    }
}

impl CreatingResponseBodyRet {
    pub fn as_ok_json(&self) -> Option<&CreatingResponseBodyOkJson> {
        match self {
            CreatingResponseBodyRet::OkJson(x) => Some(x),
            CreatingResponseBodyRet::ExtInfoError(_) => None,
        }
    }
}

//
//
//
fn parse_response(
    response: Response<Body>,
) -> Result<EndpointRet<CreatingResponseBodyRet>, EndpointError> {
    let status = response.status();
    match status {
        StatusCode::OK => Ok(EndpointRet::Ok(
            serde_json::from_slice(response.body())
                .map(CreatingResponseBodyRet::OkJson)
                .map_err(EndpointError::DeResponseBodyOkJsonFailed)?,
        )),
        status => match serde_json::from_slice::<ResponseBodyErrJson>(response.body()) {
            Ok(err_json) => {
                if let Some(err) = ExtInfoError::from_err_json(&err_json) {
                    Ok(EndpointRet::Ok(CreatingResponseBodyRet::ExtInfoError(err)))
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

//
#[derive(Serialize, Debug, Clone)]
pub struct ValueUserTag {
    pub x: f64,
    pub y: f64,
    pub username: Box<str>,
}
impl ValueUserTag {
    pub fn new(x: f64, y: f64, username: impl AsRef<str>) -> Self {
        Self {
            x,
            y,
            username: username.as_ref().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_response_body_ok_json() {
        //
        let content =
            include_str!("../../../../tests/response_body_json_files/v14.0/ig_user_0__media__creating__image.json");
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
        // TODO,
    }

    #[test]
    fn test_ext_info_error() {
        //
        let content = include_str!(
            "../../../../tests/response_body_json_files/v14.0/err__ig_user_0__media__creating__image_url_invalid.json"
        );
        match serde_json::from_str::<ResponseBodyErrJson>(content) {
            Ok(err_json) => match ExtInfoError::from_err_json(&err_json) {
                Some(ExtInfoError::ImageUrlOrVideoUrlInvalid(msg)) => {
                    println!("{}", msg);
                }
                ret => panic!("{:?}", ret),
            },
            Err(err) => panic!("{}", err),
        }

        //
        let content = include_str!(
            "../../../../tests/response_body_json_files/v14.0/err__ig_user_0__media__creating__usertag_username_private.json"
        );
        match serde_json::from_str::<ResponseBodyErrJson>(content) {
            Ok(err_json) => match ExtInfoError::from_err_json(&err_json) {
                Some(ExtInfoError::UserTagUsernamePrivateOrInvalid(msg)) => {
                    println!("{}", msg);
                }
                ret => panic!("{:?}", ret),
            },
            Err(err) => panic!("{}", err),
        }

        //
        let content = include_str!(
            "../../../../tests/response_body_json_files/v14.0/err__ig_user_0__media__creating__image_aspect_ratio_invalid.json"
        );
        match serde_json::from_str::<ResponseBodyErrJson>(content) {
            Ok(err_json) => match ExtInfoError::from_err_json(&err_json) {
                Some(ExtInfoError::ImageAspectRatioInvalid(msg)) => {
                    println!("{}", msg);
                }
                ret => panic!("{:?}", ret),
            },
            Err(err) => panic!("{}", err),
        }

        //
        let content = include_str!(
            "../../../../tests/response_body_json_files/v14.0/err__ig_user_0__media__creating__reels_account_not_allowed.json"
        );
        match serde_json::from_str::<ResponseBodyErrJson>(content) {
            Ok(err_json) => match ExtInfoError::from_err_json(&err_json) {
                Some(ExtInfoError::AccountNotAllowedCreate(msg)) => {
                    println!("{}", msg);
                }
                ret => panic!("{:?}", ret),
            },
            Err(err) => panic!("{}", err),
        }
    }
}
