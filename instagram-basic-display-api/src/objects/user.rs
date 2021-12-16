use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;

use super::AccountType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub account_type: AccountType,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
    pub username: String,
    #[serde(default)]
    pub media_count: Option<usize>,
}
