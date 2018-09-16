#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate url;


pub use self::client::TranslateAPI;
pub use self::response::{LangsResponse, DetectResponse, TranslateResponse};
pub use self::error::{Error, ApiError, Result};


mod client;
mod response;
mod error;
