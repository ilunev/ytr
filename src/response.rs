use serde::de::{Deserialize, Deserializer, DeserializeOwned};
use std::collections::HashMap;



/// API method response.
pub trait ApiResponse: DeserializeOwned {}



/// Response from `getLangs` API method.
#[derive(Deserialize)]
pub struct LangsResponse {

    /// Available translation directions.
    pub dirs: Vec<String>,

    /// `{code: name}` pairs of available languages.
    /// `None` if `ui` parameter is not set.
    pub langs: Option<HashMap<String, String>>,
}

impl ApiResponse for LangsResponse {}



/// Response from `detect` API method.
#[derive(Deserialize)]
pub struct DetectResponse {

    /// Detected language code.
    pub lang: String,
}

impl ApiResponse for DetectResponse {}



/// Response from `translate` API method.
pub struct TranslateResponse {

    /// Translated text.
    pub text: String,

    /// Source-Target language codes pair.
    pub lang: String,

    /// Detected language code.
    /// `None` if `options` parameter is not set.
    pub detected: Option<String>,
}

impl<'de> Deserialize<'de> for TranslateResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Nested {
            text: Vec<String>,
            lang: String,
            detected: Option<Detected>,
        }

        #[derive(Deserialize)]
        struct Detected {
            lang: String,
        }

        let mut nested = Nested::deserialize(deserializer)?;

        Ok(TranslateResponse {
            lang: nested.lang,
            text: nested.text.pop().unwrap(),
            detected: match nested.detected {
                Some(d) => Some(d.lang),
                None => None,
            }
        })
    }
}

impl ApiResponse for TranslateResponse {}



#[cfg(test)]
mod tests {
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
        let parsed: LangsResponse = serde_json::from_str(json)
            .unwrap();
        assert_eq!(dirs, parsed.dirs);
        assert_eq!(langs, parsed.langs.unwrap());
    }

    #[test]
    fn deserialize_detect_response() {
        let json = r#"{
                       "code": 200,
                       "lang": "en"
                      }"#;
        let parsed: DetectResponse = serde_json::from_str(json)
            .unwrap();
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
        let parsed: TranslateResponse = serde_json::from_str(json)
            .unwrap();
        assert_eq!("hello".to_string(), parsed.text);
        assert_eq!("ru-en".to_string(), parsed.lang);
        assert_eq!("ru".to_string(), parsed.detected.unwrap());
    }
}
