use serde::{Deserialize, Deserializer};
use std::collections::HashMap;


#[derive(Deserialize)]
pub struct LangsResponse {
    pub dirs: Vec<String>,
    pub langs: HashMap<String, String>,
}


#[derive(Deserialize)]
pub struct DetectResponse {
    pub lang: String,
}


pub struct TranslateResponse {
    pub text: String,
    pub lang: String,
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
