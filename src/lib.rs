#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate url;


pub use self::client::TranslateAPI;
pub use self::request::{ApiRequest, LangsRequest, DetectRequest, TranslateRequest};
pub use self::response::{ApiResponse, LangsResponse, DetectResponse, TranslateResponse};
pub use self::error::{Error, ApiError, Result};


mod client;
mod request;
mod response;
mod error;
