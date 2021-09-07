pub mod client;
pub mod request;
pub mod routing;
pub mod error;
pub mod utils;
mod berror;
use error::Error as HttpError;
use berror::Error;
use reqwest::Method;
use std::env;
pub type JsonMap = Map<String, Value>;

pub use std::result::Result as StdResult;

pub use serde_json::{Map, Number, Value};
pub type Result<T> = StdResult<T, Error>;


pub fn api(uri: String, url: String) -> String{
    format!("{}{}",uri, url)
}


pub enum LightMethod {
    /// Indicates that a route is for the `DELETE` method only.
    Delete,
    /// Indicates that a route is for the `GET` method only.
    Get,
    /// Indicates that a route is for the `PATCH` method only.
    Patch,
    /// Indicates that a route is for the `POST` method only.
    Post,
    /// Indicates that a route is for the `PUT` method only.
    Put,
}

impl LightMethod {
    pub fn reqwest_method(self) -> Method {
        match self {
            LightMethod::Delete => Method::DELETE,
            LightMethod::Get => Method::GET,
            LightMethod::Patch => Method::PATCH,
            LightMethod::Post => Method::POST,
            LightMethod::Put => Method::PUT,
        }
    }
}