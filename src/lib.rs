#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate reqwest;
extern crate url;


pub use self::client::TranslateAPI;
pub use self::response::{LangsResponse, DetectResponse, TranslateResponse};


mod client;
mod response;
