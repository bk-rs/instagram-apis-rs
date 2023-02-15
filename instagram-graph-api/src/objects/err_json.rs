use facebook_graph_api_object_error::Error;
use serde::{Deserialize, Serialize};

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ErrJson {
    pub error: Error,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_err_json() {
        //
        let content = include_str!(
            "../../tests/response_body_json_files/v14.0/err__access_token_invalid_1.json"
        );
        match serde_json::from_str::<ErrJson>(content) {
            Ok(err_json) => {
                // println!("{:?}", err_json);
                assert_eq!(
                    err_json.error.message,
                    "The access token could not be decrypted"
                );
            }
            Err(err) => panic!("{}", err),
        }
    }
}
