use serde::ser::Serialize;
use std::borrow::Borrow;

use client::TranslateAPI;
use response::{LangsResponse, DetectResponse, TranslateResponse};
use error::Result;



pub trait ApiRequest: Serialize {
    fn method(&self) -> &str;
}




#[derive(Serialize)]
pub struct LangsRequest<'a> {
    #[serde(skip)]
    client: &'a TranslateAPI,
    ui: Option<&'a str>,
}

impl<'a> LangsRequest<'a> {
    pub fn ui(mut self, value: &'a str) -> LangsRequest<'a> {
        self.ui = Some(value);
        self
    }

    pub fn get(self) -> Result<LangsResponse> {
        self.client.execute(self)
    }

    pub(crate) fn new(client: &'a TranslateAPI) -> LangsRequest<'a> {
        LangsRequest {
            client,
            ui: None
        }
    }
}

impl<'a> ApiRequest for LangsRequest<'a> {
    fn method(&self) -> &str {
        "getLangs"
    }
}




#[derive(Serialize)]
pub struct DetectRequest<'a> {
    #[serde(skip)]
    client: &'a TranslateAPI,
    text: &'a str,
    hint: Option<String>,
}

impl<'a> DetectRequest<'a> {
    pub fn hint<S>(mut self, value: &[S]) -> DetectRequest<'a>
        where S: Borrow<str>,
    {
        self.hint = Some(value.join(","));
        self
    }

    pub fn get(self) -> Result<DetectResponse> {
        self.client.execute(self)
    }

    pub(crate) fn new(client: &'a TranslateAPI, text: &'a str) -> DetectRequest<'a> {
        DetectRequest {
            client,
            text,
            hint: None,
        }
    }
}

impl<'a> ApiRequest for DetectRequest<'a> {
    fn method(&self) -> &str {
        "detect"
    }
}




#[derive(Serialize)]
pub struct TranslateRequest<'a> {
    #[serde(skip)]
    client: &'a TranslateAPI,
    text: &'a str,
    lang: &'a str,
    format: Option<&'a str>,
    options: Option<u8>,
}

impl<'a> TranslateRequest<'a> {
    pub fn format(mut self, value: &'a str) -> TranslateRequest<'a> {
        self.format = Some(value);
        self
    }

    pub fn options(mut self, value: u8) -> TranslateRequest<'a> {
        self.options = Some(value);
        self
    }

    pub fn get(self) -> Result<TranslateResponse> {
        self.client.execute(self)
    }

    pub(crate) fn new(
        client: &'a TranslateAPI,
        text: &'a str,
        lang: &'a str
    ) -> TranslateRequest<'a> {

        TranslateRequest {
            client,
            text,
            lang,
            format: None,
            options: None,
        }
    }
}

impl<'a> ApiRequest for TranslateRequest<'a> {
    fn method(&self) -> &str {
        "translate"
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use serde_urlencoded::to_string;

    #[test]
    fn langs_request_without_optional_arguments() {
        let api = TranslateAPI::new("token".to_string());
        let req = LangsRequest::new(&api);
        assert_eq!("".to_string(), to_string(&req).unwrap());
        assert_eq!("getLangs", req.method());
    }

    #[test]
    fn detect_request_with_comma_separated_hint_list() {
        let api = TranslateAPI::new("token".to_string());
        let req = DetectRequest::new(&api, "hello")
            .hint(&vec!["en", "es", "de"]);
        assert_eq!(
            "text=hello&hint=en%2Ces%2Cde".to_string(),
            to_string(&req).unwrap()
        );
        assert_eq!("detect", req.method());
    }

    #[test]
    fn translate_request_with_optional_arguments() {
        let api = TranslateAPI::new("token".to_string());
        let req = TranslateRequest::new(&api, "hello", "ru")
            .format("plain")
            .options(1);
        assert_eq!(
            "text=hello&lang=ru&format=plain&options=1".to_string(),
            to_string(&req).unwrap()
        );
        assert_eq!("translate", req.method());
    }
}
