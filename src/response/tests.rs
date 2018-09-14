use super::*;
use serde_json;
use std::collections::HashMap;


#[test]
fn deserialize_langs_response() {
    let json = r#"{
                   "code": 200,
                   "dirs": [
                       "en-ru",
                       "ru-en"
                    ],
                   "langs": {
                             "en": "English",
                             "ru": "Russian"
                            }
                   }"#;
    let dirs = vec!["en-ru".to_string(), "ru-en".to_string()];
    let langs = {
        let mut map = HashMap::new();
        map.insert("en".to_string(), "English".to_string());
        map.insert("ru".to_string(), "Russian".to_string());
        map
    };
    let parsed: LangsResponse = serde_json::from_str(json).unwrap();
    assert_eq!(dirs, parsed.dirs);
    assert_eq!(langs, parsed.langs);
}

#[test]
fn deserialize_detect_response() {
    let json = r#"{
                   "code": 200,
                   "lang": "en"
                  }"#;
    let parsed: DetectResponse = serde_json::from_str(json).unwrap();
    assert_eq!("en".to_string(), parsed.lang);
}

#[test]
fn deserialize_translate_response() {
    let json = r#"{
                   "code": 200,
                   "detected": {
                       "lang":"ru"
                    },
                    "lang": "ru-en",
                    "text": ["hello"]
                   }"#;
    let parsed: TranslateResponse = serde_json::from_str(json).unwrap();
    assert_eq!("hello".to_string(), parsed.text);
    assert_eq!("ru-en".to_string(), parsed.lang);
    assert_eq!("ru".to_string(), parsed.detected.unwrap());
}
