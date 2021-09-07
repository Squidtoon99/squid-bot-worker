use std::{
    borrow::Cow
};
use super::{LightMethod, api};

// macro_rules! api {
//      ($e:expr) => {
//         concat!(format!("{}", env::var("upstash_uri")), $e)
//     };
//     ($e:expr, $($rest:tt)*) => {
//         format!(api!($e), $($rest)*)
//     };
// }

pub enum Route{}

impl Route {
    pub fn get(uri: String, key: &str) -> String {
        api(uri, format!("/get/{}", key))
    }

    pub fn set(uri: String, key: &str, value: &str) -> String {
        api(uri, format!("/set/{}/{}", key, value))
    }

    pub fn incr(uri: String, key: &str) -> String {
        api(uri, format!("/incr/{}", key))
    }

}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub(crate) enum RouteInfo<'a> {
    Set {
        key: &'a str,
        value: &'a str
    },

    Get {
        key: &'a str
    },

    Incr {
        key: &'a str
    }
}

impl <'a> RouteInfo<'a> {
    pub fn deconstruct(&self, uri: String) -> (LightMethod, Cow<'_, str>){
        match *self {
            RouteInfo::Set { key, value } => (
                LightMethod::Post,
                Cow::from(Route::set(uri, key, value))
            ),
            RouteInfo::Get { key } => (
                LightMethod::Get,
                Cow::from(Route::get(uri, key))
            ),
            RouteInfo::Incr { key } => (
                LightMethod::Get,
                Cow::from(Route::incr(uri, key))
            )
        }
    }
}