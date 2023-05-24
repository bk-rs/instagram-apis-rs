//! [Ref](https://developers.facebook.com/docs/instagram-api/reference/ig-comment#fields)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::{
    deserialize_number_from_string, deserialize_option_number_from_string,
};

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IgCommentForIgMediaCommentsReadingOperation {
    pub from: Option<IgCommentFrom>,
    pub hidden: bool,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
    pub like_count: u32,
    #[serde(default)]
    pub text: Box<str>,
    pub timestamp: DateTime<Utc>,
    pub username: Box<str>,
    #[serde(default)]
    pub replies: IgCommentReplies,
}

impl IgCommentForIgMediaCommentsReadingOperation {
    pub fn fields() -> Box<str> {
        "from,hidden,id,like_count,text,timestamp,username,replies{from,hidden,id,like_count,parent_id,text,timestamp,username}".into()
    }
}

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IgCommentForIgMediaCommentsCreatingOperation {
    pub from: Option<IgCommentFrom>,
    pub hidden: bool,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
    pub like_count: u32,
    #[serde(default)]
    pub text: Box<str>,
    pub timestamp: DateTime<Utc>,
    pub username: Box<str>,
}

impl IgCommentForIgMediaCommentsCreatingOperation {
    pub fn fields() -> Box<str> {
        "from,hidden,id,like_count,text,timestamp,username".into()
    }
}

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IgCommentForIgCommentReadingOperation {
    pub from: Option<IgCommentFrom>,
    pub hidden: bool,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
    pub like_count: u32,
    pub media: IgCommentMedia,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub parent_id: Option<u64>,
    #[serde(default)]
    pub text: Box<str>,
    pub timestamp: DateTime<Utc>,
    pub username: Box<str>,
    #[serde(default)]
    pub replies: IgCommentReplies,
}

impl IgCommentForIgCommentReadingOperation {
    pub fn fields() -> Box<str> {
        "from,hidden,id,like_count,media,parent_id,text,timestamp,username".into()
    }

    pub fn fields_with_replies() -> Box<str> {
        "from,hidden,id,like_count,media,parent_id,text,timestamp,username,replies{from,hidden,id,like_count,parent_id,text,timestamp,username}".into()
    }
}

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IgCommentFrom {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
    pub username: Box<str>,
}

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IgCommentMedia {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
    pub media_product_type: Option<Box<str>>,
}

//
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct IgCommentReplies {
    pub data: Vec<IgCommentAsReply>,
}

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IgCommentAsReply {
    pub from: Option<IgCommentFrom>,
    pub hidden: bool,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
    pub like_count: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub parent_id: u64,
    #[serde(default)]
    pub text: Box<str>,
    pub timestamp: DateTime<Utc>,
    pub username: Box<str>,
}

impl IgCommentAsReply {
    pub fn fields() -> Box<str> {
        "from,hidden,id,like_count,parent_id,text,timestamp,username".into()
    }
}
