use serde::ser::Serialize;
use std::borrow::Borrow;
use std::fmt;

use crate::{
    client::ApiClient,
    response::{LangsResponse, DetectResponse, TranslateResponse},
    error::Result,
};



/// Request to be executed.
pub trait ApiRequest: Serialize {
    /// Method name for url.
    fn method(&self) -> &str;
}



/// Parameters for `getLangs` API method.
///
/// Instances are created via [`ApiClient::get_langs`].
///
/// [`ApiClient::get_langs`]: struct.ApiClient.html#method.get_langs
#[derive(Serialize)]
pub struct LangsRequest<'a> {
    #[serde(skip)]
    client: &'a ApiClient,
    ui: Option<&'a str>,
}


impl<'a> LangsRequest<'a> {
    /// Set `ui` parameter.
    pub fn ui(mut self, ui: &'a str) -> LangsRequest<'a> {
        self.ui = Some(ui);
        self
    }

    /// Call API method with prepared parameters.
    pub fn get(self) -> Result<LangsResponse> {
        self.client.execute(self)
    }

    pub(crate) fn new(client: &'a ApiClient) -> LangsRequest<'a> {
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


impl<'a> fmt::Debug for LangsRequest<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("LangsRequest")
            .field("ui", &self.ui)
            .finish()
    }
}




/// Parameters for `detect` API method.
///
/// Instances are created via [`ApiClient::detect`].
///
/// [`ApiClient::detect`]: struct.ApiClient.html#method.detect
#[derive(Serialize)]
pub struct DetectRequest<'a> {
    #[serde(skip)]
    client: &'a ApiClient,
    text: &'a str,
    hint: Option<String>,
}


impl<'a> DetectRequest<'a> {
    /// Set `hint` parameter.
    pub fn hint<S>(mut self, hint: &[S]) -> DetectRequest<'a>
        where S: Borrow<str>,
    {
        self.hint = Some(hint.join(","));
        self
    }

    /// Call API method with prepared parameters.
    pub fn get(self) -> Result<DetectResponse> {
        self.client.execute(self)
    }

    pub(crate) fn new(client: &'a ApiClient, text: &'a str) -> DetectRequest<'a> {
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


impl<'a> fmt::Debug for DetectRequest<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("DetectRequest")
            .field("text", &self.text)
            .field("hint", &self.hint)
            .finish()
    }
}




/// Parameters for `translate` API method.
///
/// Instances are created via [`ApiClient::translate`].
///
/// [`ApiClient::translate`]: struct.ApiClient.html#method.translate
#[derive(Serialize)]
pub struct TranslateRequest<'a> {
    #[serde(skip)]
    client: &'a ApiClient,
    text: &'a str,
    lang: &'a str,
    format: Option<&'a str>,
    options: Option<u8>,
}


impl<'a> TranslateRequest<'a> {
    /// Set `format` parameter.
    pub fn format(mut self, format: &'a str) -> TranslateRequest<'a> {
        self.format = Some(format);
        self
    }

    /// Set `options` parameter.
    pub fn options(mut self, options: u8) -> TranslateRequest<'a> {
        self.options = Some(options);
        self
    }

    /// Call API method with prepared parameters.
    pub fn get(self) -> Result<TranslateResponse> {
        self.client.execute(self)
    }

    pub(crate) fn new(
        client: &'a ApiClient,
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


impl<'a> fmt::Debug for TranslateRequest<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TranslateRequest")
            .field("text", &self.text)
            .field("lang", &self.lang)
            .field("format", &self.format)
            .field("options", &self.options)
            .finish()
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use serde_urlencoded::to_string;

    #[test]
    fn langs_request_without_optional_arguments() {
        let api = ApiClient::new("token".to_string());
        let req = LangsRequest::new(&api);
        assert_eq!("".to_string(), to_string(&req).unwrap());
        assert_eq!("getLangs", req.method());
    }

    #[test]
    fn detect_request_with_comma_separated_hint_list() {
        let api = ApiClient::new("token".to_string());
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
        let api = ApiClient::new("token".to_string());
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
