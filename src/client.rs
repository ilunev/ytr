use url::Url;
use reqwest::{Client, StatusCode, header::ContentType};
use serde_urlencoded;

use response::ApiResponse;
use request::{ApiRequest, LangsRequest, DetectRequest, TranslateRequest};
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


    pub fn get_langs(&self) -> LangsRequest {
        LangsRequest::new(&self)
    }


    pub fn detect<'a>(&'a self, text: &'a str) -> DetectRequest<'a> {
        DetectRequest::new(&self, text)
    }


    pub fn translate<'a>(
        &'a self,
        text: &'a str,
        lang: &'a str
    ) -> TranslateRequest<'a> {
        
        TranslateRequest::new(&self, text, lang)
    }


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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_url_test() {
        let api = TranslateAPI::new("mytoken".to_string());
        let url = api.make_url("method");
        assert_eq!(
            "https://translate.yandex.net/api/v1.5/tr.json/method?key=mytoken",
            url.as_str()
        );
    }
}
