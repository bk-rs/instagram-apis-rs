//! [Ref](https://developers.facebook.com/docs/instagram-api/reference/ig-user#fields)

use chrono::{DateTime, Utc};
use facebook_graph_api_object_paging::cursor_based_pagination::Paging;
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;

use crate::objects::IgMediaForIgUserBusinessDiscoveryReadingOperation;

//
//
//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IgUserForIgUserBusinessDiscoveryReadingOperation {
    pub biography: Option<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub ig_id: u64,
    pub followers_count: u32,
    pub follows_count: u32,
    pub media_count: u32,
    pub name: Option<String>,
    pub profile_picture_url: Option<String>,
    // TODO, "(#10) Application does not have permission for this action"
    // pub shopping_product_tag_eligibility: bool,
    pub username: String,
    pub website: Option<String>,

    #[serde(default)]
    pub media: IgUserMediaForIgUserBusinessDiscoveryReadingOperation,
    /*
    live_media "(#36104) Cannot access live media from business discovery"
    stories "(#10) Application does not have permission for this action"
    */
}

impl IgUserForIgUserBusinessDiscoveryReadingOperation {
    pub fn fields(media_since: Option<DateTime<Utc>>, media_limit: Option<usize>) -> Box<str> {
        let media_since = if let Some(media_since) = media_since {
            format!(".since({})", media_since.timestamp())
        } else {
            "".into()
        };
        let media_limit = if let Some(media_limit) = media_limit {
            format!(".limit({})", media_limit)
        } else {
            "".into()
        };

        format!("biography,id,ig_id,followers_count,follows_count,media_count,name,profile_picture_url,username,website,media{}{}{{caption,comments_count,id,like_count,media_product_type,media_type,media_url,permalink,timestamp,children{{id,media_type,media_url,permalink,timestamp}}}}", media_since, media_limit).into()
    }
}

//
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct IgUserMediaForIgUserBusinessDiscoveryReadingOperation {
    pub data: Vec<IgMediaForIgUserBusinessDiscoveryReadingOperation>,
    pub paging: Option<Paging>,
}
