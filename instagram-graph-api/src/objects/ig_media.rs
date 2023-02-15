//! [Ref](https://developers.facebook.com/docs/instagram-api/reference/ig-media#fields)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;

//
//
//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IgMediaForIgUserBusinessDiscoveryReadingOperation {
    pub caption: Option<String>,
    pub comments_count: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
    // TODO, "(#100) Please read documentation for supported fields."
    // pub ig_id: u64,
    // TODO, "(#100) Please read documentation for supported fields."
    // pub is_comment_enabled: bool,
    pub like_count: u32,
    pub media_product_type: IgMediaProductType,
    pub media_type: IgMediaType,
    pub media_url: Option<String>,
    pub permalink: String,
    // TODO, "(#100) Please read documentation for supported fields."
    // pub shortcode: Box<str>,
    // TODO, "(#100) Please read documentation for supported fields."
    // pub thumbnail_url: Option<Box<str>>,
    pub timestamp: DateTime<Utc>,
    //
    pub children: Option<IgMediaChildrenForIgUserBusinessDiscoveryReadingOperation>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IgMediaChildrenForIgUserBusinessDiscoveryReadingOperation {
    pub data: Vec<IgMediaChildForIgUserBusinessDiscoveryReadingOperation>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IgMediaChildForIgUserBusinessDiscoveryReadingOperation {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
    pub media_type: IgMediaType,
    pub media_url: Option<String>,
    pub permalink: String,
    pub timestamp: DateTime<Utc>,
}

//
//
//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IgMediaForIgUserMediaPublishCreatingOperation {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
}

impl IgMediaForIgUserMediaPublishCreatingOperation {
    pub fn fields() -> Box<str> {
        "id".into()
    }
}

//
//
//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum IgMediaProductType {
    #[serde(rename = "AD")]
    Ad,
    #[serde(rename = "FEED")]
    Feed,
    #[serde(rename = "STORY")]
    Story,
    #[serde(rename = "IGTV")]
    Igtv,
    #[serde(rename = "REELS")]
    Reels,
}

impl Default for IgMediaProductType {
    fn default() -> Self {
        Self::Feed
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum IgMediaType {
    #[serde(rename = "IMAGE")]
    Photo,
    #[serde(rename = "VIDEO")]
    Video,
    #[serde(rename = "CAROUSEL_ALBUM")]
    Album,
}

impl Default for IgMediaType {
    fn default() -> Self {
        Self::Photo
    }
}
