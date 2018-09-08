use super::*;

#[test]
fn url_path_from_make_url() {
    let token = String::from("apikey");
    let api = TranslateAPI::new(token);
    let url = api.make_url("getLangs", vec![("ui", "en")]);
    let url = url.as_str();
    assert!(url.starts_with("https://translate.yandex.net/api/v1.5/tr.json/getLangs?"));
}

#[test]
fn url_query_from_make_url() {
    let token = String::from("trnsl.token");
    let api = TranslateAPI::new(token);
    let url = api.make_url("detect", vec![("text", "sometext"), ("hint", "en")]);
    let url = url.as_str();
    assert!(
        url.contains("key=trnsl.token")
        && url.contains("text=sometext")
        && url.contains("hint=en")
    );
}

#[test]
fn make_url_query_quoting() {
    let token = String::from("mytoken");
    let api = TranslateAPI::new(token);
    let url = api.make_url("translate", vec![("text", "Эм&Эмс"), ("lang", "en")]);
    let url = url.as_str();
    assert!(url.contains("text=%D0%AD%D0%BC%26%D0%AD%D0%BC%D1%81"));
}
