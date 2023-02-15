//! [Ref](https://developers.facebook.com/docs/instagram-api/reference/ig-container#fields)

use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IgContainer {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
    pub status: Box<str>,
    pub status_code: IgContainerStatusCode,
}

impl IgContainer {
    pub fn fields() -> Box<str> {
        "id,status,status_code".into()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IgContainerStatusCode {
    Expired,
    Error,
    Finished,
    InProgress,
    Published,
}
