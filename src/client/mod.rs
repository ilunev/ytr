use url::Url;
use serde::de::DeserializeOwned;
use reqwest::{Client, StatusCode};
use std::borrow::Borrow;

use error::Error;

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

    fn request<T>(&self, url: Url) -> Result<T, Error>
        where T: DeserializeOwned,
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
}


#[cfg(test)]
mod tests;
