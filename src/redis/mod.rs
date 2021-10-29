mod berror;
pub mod client;
pub mod error;
pub mod request;
pub mod routing;
pub mod utils;
use berror::Error;
use error::Error as HttpError;
use reqwest::Method;
pub type JsonMap = Map<String, Value>;

pub(crate) use client::RedisClient;
pub use std::result::Result as StdResult;

pub use serde_json::{Map, Number, Value};
pub type Result<T> = StdResult<T, Error>;

pub fn api(uri: String, url: String) -> String {
    format!("{}{}", uri, url)
}

pub enum LightMethod {
    /// Indicates that a route is for the `GET` method only.
    Get,
    /// Indicates that a route is for the `POST` method only.
    Post,
}

impl LightMethod {
    pub fn reqwest_method(self) -> Method {
        match self {
            LightMethod::Get => Method::GET,
            LightMethod::Post => Method::POST,
        }
    }
}
