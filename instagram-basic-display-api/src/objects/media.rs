use chrono::{DateTime, Utc};
use instagram_link::MediaLink;
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Media {
    pub caption: Option<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
    pub media_type: MediaType,
    pub media_url: Option<String>,
    pub permalink: String,
    pub thumbnail_url: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub username: String,
    pub children: Option<MediaChildren>,
}
impl Media {
    pub fn get_ig_id_and_shortcode(&self) -> Option<(u64, String)> {
        get_ig_id_and_shortcode(&self.permalink).ok()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaChildren {
    pub data: Vec<MediaCarouselAlbumChild>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaCarouselAlbumChild {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
    pub media_type: MediaType,
    pub media_url: Option<String>,
    pub permalink: String,
    pub thumbnail_url: Option<String>,
    pub timestamp: DateTime<Utc>,
}
impl MediaCarouselAlbumChild {
    pub fn get_ig_id_and_shortcode(&self) -> Option<(u64, String)> {
        get_ig_id_and_shortcode(&self.permalink).ok()
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum MediaType {
    #[serde(rename = "IMAGE")]
    Photo,
    #[serde(rename = "VIDEO")]
    Video,
    #[serde(rename = "CAROUSEL_ALBUM")]
    Album,
}

//
pub fn get_ig_id_and_shortcode(permalink: &str) -> Result<(u64, String), String> {
    let media_link = MediaLink::parse(permalink).map_err(|err| err.to_string())?;

    let metadata = match media_link {
        MediaLink::Post { metadata } => metadata,
        MediaLink::Story {
            metadata,
            owner_username: _,
        } => metadata,
        MediaLink::StoryHighlight {
            metadata,
            highlight_id: _,
        } => metadata,
        MediaLink::IGTVVideo { metadata } => metadata,
        MediaLink::Reel { metadata } => metadata,
    };

    Ok((metadata.ig_id, metadata.shortcode))
}
