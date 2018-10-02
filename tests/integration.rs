extern crate ytr;
use ytr::ApiClient;
use ytr::Error;

use std::fs;


fn valid_client() -> ApiClient {
    let key = fs::read_to_string(".testkey")
        .expect("need '.testkey' file with API key to run integration tests");

    ApiClient::new(key)
}

#[test]
#[ignore]
fn test_translate() {
    let api = valid_client();
    let res = api
        .translate("Hi!", "ru")
        .options(1)
        .get()
        .unwrap();
    assert_eq!("Привет!", res.text);
    assert_eq!("en-ru", res.lang);
    assert_eq!("en", res.detected.unwrap());
}

#[test]
#[ignore]
fn test_detect() {
    let api = valid_client();
    let res = api
        .detect("Hello world!")
        .hint(&["es"])
        .get()
        .unwrap();
    assert_eq!("en", res.lang);
}

#[test]
#[ignore]
fn test_langs() {
    let api = valid_client();
    let res = api
        .get_langs()
        .ui("en")
        .get()
        .unwrap();
    assert_eq!("Russian", res.langs.unwrap().get("ru").unwrap());
}

#[test]
#[ignore]
fn test_with_invalid_key() -> Result<(), String> {
    let api = ApiClient::new("invalid.token".to_string());
    let res = api.get_langs().get();
    match res {
        Ok(_) => Err("request succeeded with invalid API key".into()),
        Err(err) => match err {
            Error::ApiError(_) => Ok(()),
            _ => Err(format!("expected Error::ApiError, got {:?}", err))
        }
    }
}
