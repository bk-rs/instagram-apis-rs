use facebook_graph_api_object_error::{Error, KnownErrorCase};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseErrorBody {
    pub error: Error,
}

impl ResponseErrorBody {
    #[deprecated(
        since = "0.2.1",
        note = "use `.error.is_access_token_session_has_expired()` instead"
    )]
    pub fn is_access_token_session_has_expired(&self) -> bool {
        self.error.is_access_token_session_has_expired()
    }

    #[deprecated(
        since = "0.2.1",
        note = "use `.error.is_access_token_session_has_been_invalidated()` instead"
    )]
    pub fn is_access_token_session_has_been_invalidated(&self) -> bool {
        self.error.is_access_token_session_has_been_invalidated()
    }

    #[deprecated(
        since = "0.2.1",
        note = "use `.error.to_known_error_case().map(|x| matches!(x, KnownErrorCase::PermissionNotGrantedOrRemoved)) == Some(true)` instead"
    )]
    pub fn is_not_have_permission(&self) -> bool {
        self.error
            .to_known_error_case()
            .map(|x| matches!(x, KnownErrorCase::PermissionNotGrantedOrRemoved))
            == Some(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_response_error_body() {
        //
        let body = serde_json::from_str::<ResponseErrorBody>(include_str!(
            "../../tests/response_body_files/exchange_sl_access_token_for_ll_access_token_err__400.json"
        ))
        .unwrap();
        assert!(body.error.is_access_token_session_has_expired());

        let body = serde_json::from_str::<ResponseErrorBody>(include_str!(
            "../../tests/response_body_files/me_err__400.json"
        ))
        .unwrap();
        assert!(body.error.is_access_token_session_has_expired());

        //
        let body = serde_json::from_str::<ResponseErrorBody>(include_str!(
            "../../tests/response_body_files/err__1.json"
        ))
        .unwrap();
        assert!(
            body.error
                .to_known_error_case()
                .map(|x| matches!(x, KnownErrorCase::PermissionNotGrantedOrRemoved))
                == Some(true)
        );

        //
        let _body = serde_json::from_str::<ResponseErrorBody>(include_str!(
            "../../tests/response_body_files/err__2.json"
        ))
        .unwrap();

        //
        let body = serde_json::from_str::<ResponseErrorBody>(include_str!(
            "../../tests/response_body_files/err__3.json"
        ))
        .unwrap();
        assert!(body.error.is_access_token_session_has_been_invalidated());
    }
}
