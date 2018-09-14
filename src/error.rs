use std::io::Read;


pub enum Error {
    ApiError(ApiError),
    UnexpectedResponse(String),
    RequestFailed,
}

impl Error {
    pub fn unexpected_from_reader<R>(reader: &mut R) -> Error
        where R: Read,
    {
        let mut content = String::new();
        let _ = reader.read_to_string(&mut content);            
        Error::UnexpectedResponse(content)
    }
}


#[derive(Deserialize)]
pub struct ApiError {
    pub code: u16,
    pub message: String,
}




#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use std::io::{Cursor};

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

    #[test]
    fn unexpected_response_constructor() {
        let mut file = Cursor::new("error".to_string());
        let error = Error::unexpected_from_reader(&mut file);
        if let Error::UnexpectedResponse(content) = error {
            assert_eq!("error".to_string(), content);
        } else {
            panic!("Wrong enum variant");
        }
    }
}
