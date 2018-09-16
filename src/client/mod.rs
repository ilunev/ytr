use url::Url;
use reqwest::{Client, StatusCode};
use std::borrow::Borrow;

use response::{ApiResponse, LangsResponse, DetectResponse, TranslateResponse};
use error::{Error, Result};

const BASE_URL: &str = "https://translate.yandex.net/api/v1.5/tr.json";



pub struct TranslateAPI {
    key: String,
    client: Client,
}

impl TranslateAPI {
    pub fn new(key: String) -> TranslateAPI {
        let client = Client::new();
        TranslateAPI {
            key,
            client,
        }
    }

    pub fn get_langs(&self, ui: &str) -> Result<LangsResponse> {
        let params = vec![("ui", ui)];
        let url = self.make_url("getLangs", params);
        self.request(url)
    }

    pub fn detect<S>(&self, text: &str, hint: Option<&[S]>) -> Result<DetectResponse>
        where S: Borrow<str>,
    {
        let joined_hint;
        let mut params = vec![("text", text)];
        if let Some(hint) = hint {
            joined_hint = hint.join(",");
            params.push(("hint", &joined_hint));
        }
        let url = self.make_url("detect", params);
        self.request(url)
    }

    pub fn translate(&self, text: &str, lang: &str, format: Option<&str>, options: Option<u8>) -> Result<TranslateResponse> {
        let string_options;
        let mut params = vec![("text", text), ("lang", lang)];
        if let Some(format) = format {
            params.push(("format", format));
        }
        if let Some(options) = options {
            string_options = options.to_string();
            params.push(("options", &string_options));
        }
        let url = self.make_url("translate", params);
        self.request(url)
    }

    fn make_url<I, K, V>(&self, method: &str, params: I) -> Url
        where I: IntoIterator,
              I::Item: Borrow<(K, V)>,
              K: AsRef<str>,
              V: AsRef<str>,
    {
        let mut url = Url::parse(BASE_URL).unwrap();
        url.path_segments_mut()
            .unwrap()
            .push(method);
        url.query_pairs_mut()
            .append_pair("key", &self.key)
            .extend_pairs(params);
        url
    }

    fn request<T>(&self, url: Url) -> Result<T>
        where T: ApiResponse,
    {
        let req_res = self.client
            .get(url)
            .send();

        if let Err(_) = req_res {
            return Err(Error::RequestFailed);
        }

        let mut resp = req_res.unwrap();

        if let StatusCode::Ok = resp.status() {
            if let Ok(obj) = resp.json() {
                Ok(obj)
            } else {
                Err(Error::UnexpectedResponse)
            }
        } else {
            if let Ok(err_obj) = resp.json() {
                Err(Error::ApiError(err_obj))
            } else {
                Err(Error::UnexpectedResponse)
            }
        }
    }
}


#[cfg(test)]
mod tests;
