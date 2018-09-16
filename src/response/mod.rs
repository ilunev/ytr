use serde::de::{Deserialize, Deserializer, DeserializeOwned};
use std::collections::HashMap;


pub trait ApiResponse: DeserializeOwned {}


#[derive(Deserialize)]
pub struct LangsResponse {
    pub dirs: Vec<String>,
    pub langs: Option<HashMap<String, String>>,
}

impl ApiResponse for LangsResponse {}


#[derive(Deserialize)]
pub struct DetectResponse {
    pub lang: String,
}

impl ApiResponse for DetectResponse {}


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

impl ApiResponse for TranslateResponse {}


#[cfg(test)]
mod tests;
