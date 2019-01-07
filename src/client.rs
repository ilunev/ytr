use std::fmt;

use url::Url;
use reqwest::{Client, StatusCode, header::ContentType};
use serde_urlencoded;

use crate::{
    response::ApiResponse,
    request::{
        ApiRequest,
        LangsRequest,
        DetectRequest,
        TranslateRequest
    },
    error::{
        Error,
        Result
    },
};


const BASE_URL: &str = "https://translate.yandex.net/api/v1.5/tr.json";



/// Client to call API methods.
pub struct ApiClient {
    key: String,
    client: Client,
}

impl ApiClient {
    /// Create new `ApiClient` with the specified API key.
    ///
    /// # Panic
    /// Panics if native TLS backend cannot be created or initialized.
    pub fn new(key: String) -> ApiClient {
        let client = Client::new();
        ApiClient {
            key,
            client,
        }
    }


    /// Create `LangsRequest` to call `getLangs` API method.
    pub fn get_langs(&self) -> LangsRequest {
        LangsRequest::new(&self)
    }


    /// Create `DetectRequest` to call `detect` API method.
    pub fn detect<'a>(&'a self, text: &'a str) -> DetectRequest<'a> {
        DetectRequest::new(&self, text)
    }


    /// Create `TranslateRequest` to call `translate` API method.
    pub fn translate<'a>(
        &'a self,
        text: &'a str,
        lang: &'a str
    ) -> TranslateRequest<'a> {
        
        TranslateRequest::new(&self, text, lang)
    }


    /// Execute prepared request.
    ///
    /// **Note**: this crate's [`ApiRequest`] structs have `get()` method, so that you don't need to call
    /// `execute` manually. But you can still use it to execute custom request structs that implement
    /// [`ApiRequest`].
    ///
    /// # Panic
    /// Panics if `req` cannot be serialized with `serde_urlencoded`.
    ///
    /// [`ApiRequest`]: trait.ApiRequest.html
    pub fn execute<Req, Resp>(&self, req: Req) -> Result<Resp>
        where Req: ApiRequest,
              Resp: ApiResponse,
    {
        let url = self.make_url(req.method());
        let data = serde_urlencoded::to_string(&req)
            .expect("can't serialize request contents");
        let resp = self.client
            .post(url)
            .body(data)
            .header(ContentType::form_url_encoded())
            .send();

        if let Ok(mut resp) = resp {
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
        } else {
            Err(Error::RequestFailed)
        }
    }


    fn make_url(&self, method: &str) -> Url {
        let mut url = Url::parse(BASE_URL)
            .unwrap();             // always parsed
        url.path_segments_mut()
            .unwrap()              // BASE_URL is not cannot-be-a-base
            .push(method);
        url.query_pairs_mut()
            .append_pair("key", &self.key);
        url
    }
}


impl fmt::Debug for ApiClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ApiClient")
            .finish()
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_url_test() {
        let api = ApiClient::new("mytoken".to_string());
        let url = api.make_url("method");
        assert_eq!(
            "https://translate.yandex.net/api/v1.5/tr.json/method?key=mytoken",
            url.as_str()
        );
    }
}
