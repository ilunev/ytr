/// `Result` alias with [`ytr::Error`] as the `Err` variant.
///
/// [`ytr::Error`]: enum.Error.html
pub type Result<T> = ::std::result::Result<T, Error>;



/// Possible types of errors.
#[derive(Debug)]
pub enum Error {

    /// Request succeeded, but API returned an error code.
    ApiError(ApiError),

    /// Request succeeded, but the response could not be parsed.
    UnexpectedResponse,

    /// Could not connect to server.
    RequestFailed,
}



/// API returned error.
#[derive(Deserialize, Debug)]
pub struct ApiError {

    /// Error code.
    pub code: u16,

    /// Error message.
    pub message: String,
}




#[cfg(test)]
mod tests {
    extern crate serde_json;

    use super::*;

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
