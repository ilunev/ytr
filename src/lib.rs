//! # ytr
//!
//! This crate is a [Yandex.Translate] API wrapper for Rust. It helps to use machine translations
//! in your Rust application. A detailed description of the Yandex.Translate API and its methods
//! is available in the [official documentation] (in Russian).
//!
//! ## Usage
//!
//! To work with the API you will need an [`ApiClient`] instance holding your API key.
//!
//! [`ApiClient`] defines several methods that correspond to API methods. They are called with the
//! required parameters and return a request object, which can then be configured with optional
//! parameters and executed using the `get()` method.
//!
//! The execution of the request returns `Result<T, ytr::Error>`, where T is a response object.
//! If the request was successful, the returned data will be contained in the public fields of the
//! response. Some data is returned only when the optional parameters are set, so the fields for
//! it contain the `Option<T>`.
//!
//! ## Usage example
//!
//! ```rust, no_run
//! let key = String::from("my-api-key");
//! let api = ytr::ApiClient::new(key);
//!
//! let result = api.translate("Hello!", "ru")   // required parameters
//!     .format("plain")                         // optional parameter
//!     .get();                                  // execute the request
//! 
//! match result {
//!     Ok(response) => {
//!         println!("{}", response.text);       // prints "Привет!"
//!         println!("{}", response.lang);       // prints "en-ru"
//!     },
//!     
//!     Err(error) => {
//!         println!(
//!             "An error has occured: {:?}",
//!             error
//!         );
//!     },
//! };
//! ```
//!
//! [`ApiClient`]: struct.ApiClient.html
//! [Yandex.Translate]: https://tech.yandex.ru/translate/
//! [official documentation]: https://tech.yandex.ru/translate/doc/dg/concepts/About-docpage/


#![deny(missing_docs, missing_debug_implementations, warnings)]



#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_urlencoded;
extern crate reqwest;
extern crate url;


pub use self::client::ApiClient;
pub use self::request::{ApiRequest, LangsRequest, DetectRequest, TranslateRequest};
pub use self::response::{ApiResponse, LangsResponse, DetectResponse, TranslateResponse};
pub use self::error::{Error, ApiError, Result};


mod client;
mod request;
mod response;
mod error;
