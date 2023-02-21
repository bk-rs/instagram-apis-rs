use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum AccountType {
    #[serde(rename = "BUSINESS")]
    Business,
    #[serde(rename = "MEDIA_CREATOR")]
    MediaCreator,
    #[serde(rename = "PERSONAL")]
    Personal,
}

impl Default for AccountType {
    fn default() -> Self {
        Self::Personal
    }
}
