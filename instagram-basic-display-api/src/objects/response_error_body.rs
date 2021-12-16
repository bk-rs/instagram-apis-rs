use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseErrorBody {
    pub error: ResponseErrorBodyError,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseErrorBodyError {
    pub r#type: ResponseErrorBodyErrorType,
    pub code: isize,
    pub message: String,
    pub fbtrace_id: String,
    #[serde(default)]
    pub error_subcode: Option<isize>,
}

#[derive(Serialize_enum_str, Deserialize_enum_str, Debug, Clone)]
pub enum ResponseErrorBodyErrorType {
    OAuthException,
    #[serde(other)]
    Other(String),
}

impl ResponseErrorBody {
    pub fn is_access_token_session_has_expired(&self) -> bool {
        self.error
            .message
            .to_lowercase()
            .contains("session has expired")
    }

    pub fn is_not_have_permission(&self) -> bool {
        self.error
            .message
            .to_lowercase()
            .contains("not have permission")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_response_error_body() {
        let body = serde_json::from_str::<ResponseErrorBody>(include_str!(
            "../../tests/response_body_files/exchange_sl_access_token_for_ll_access_token_err__400.json"
        ))
        .unwrap();
        assert!(body.is_access_token_session_has_expired());

        let body = serde_json::from_str::<ResponseErrorBody>(include_str!(
            "../../tests/response_body_files/err__1.json"
        ))
        .unwrap();
        assert!(body.is_not_have_permission());
    }
}
