pub type Result<T> = ::std::result::Result<T, Error>;



#[derive(Debug)]
pub enum Error {
    ApiError(ApiError),
    UnexpectedResponse,
    RequestFailed,
}



#[derive(Deserialize, Debug)]
pub struct ApiError {
    pub code: u16,
    pub message: String,
}




#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn deserialize_error_response() {
        let json = r#"{
                       "code": 401,
                       "message": "API key is invalid"
                      }"#;
        let parsed: ApiError = serde_json::from_str(json).unwrap();
        assert_eq!(401, parsed.code);
        assert_eq!("API key is invalid", parsed.message);
    }
}
