use super::{api, LightMethod};
use std::borrow::Cow;

// macro_rules! api {
//      ($e:expr) => {
//         concat!(format!("{}", env::var("upstash_uri")), $e)
//     };
//     ($e:expr, $($rest:tt)*) => {
//         format!(api!($e), $($rest)*)
//     };
// }

pub enum Route {}

#[allow(dead_code)]
impl Route {
    pub fn append(uri: String, key: &str, text: &str) -> String {
        api(uri, format!("/append/{}/{}", key, text))
    }

    pub fn decr(uri: String, key: &str) -> String {
        api(uri, format!("/decr/{}", key))
    }

    pub fn decrby(uri: String, key: &str, value: i64) -> String {
        api(uri, format!("/decrby/{}/{}", key, value))
    }

    pub fn get(uri: String, key: &str) -> String {
        api(uri, format!("/get/{}", key))
    }

    pub fn getdel(uri: String, key: &str) -> String {
        api(uri, format!("/getdel/{}", key))
    }

    pub fn getset(uri: String, key: &str, value: &str) -> String {
        api(uri, format!("/getset/{}/{}", key, value))
    }

    pub fn getrange(uri: String, key: &str, start: i64, end: i64) -> String {
        api(uri, format!("/getrange/{}/{}/{}", key, start, end))
    }

    pub fn incr(uri: String, key: &str) -> String {
        api(uri, format!("/incr/{}", key))
    }

    pub fn incrby(uri: String, key: &str, value: i64) -> String {
        api(uri, format!("/incrby/{}/{}", key, value))
    }

    pub fn set(uri: String, key: &str, value: &str) -> String {
        api(uri, format!("/set/{}/{}", key, value))
    }

    pub fn del(uri: String, key: &str) -> String {
        api(uri, format!("/del/{}", key))
    }
}

#[derive(Clone, Debug)]
#[non_exhaustive]
#[allow(dead_code)] // Writing up functions for future use
pub(crate) enum RouteInfo<'a> {
    Append { key: &'a str, text: &'a str },
    Decr { key: &'a str },
    DecrBy { key: &'a str, value: i64 },

    Get { key: &'a str },

    GetDel { key: &'a str },

    GetSet { key: &'a str, value: &'a str },

    GetRange { key: &'a str, start: i64, end: i64 },

    Incr { key: &'a str },

    IncrBy { key: &'a str, value: i64 },

    Set { key: &'a str, value: &'a str },

    Del { key: &'a str },
}

impl<'a> RouteInfo<'a> {
    pub fn deconstruct(&self, uri: String) -> (LightMethod, Cow<'_, str>) {
        match *self {
            RouteInfo::Append { key, text } => {
                (LightMethod::Post, Cow::from(Route::append(uri, key, text)))
            }
            RouteInfo::Decr { key } => (LightMethod::Get, Cow::from(Route::get(uri, key))),
            RouteInfo::DecrBy { key, value } => {
                (LightMethod::Get, Cow::from(Route::decrby(uri, key, value)))
            }

            RouteInfo::Get { key } => (LightMethod::Get, Cow::from(Route::get(uri, key))),
            RouteInfo::GetDel { key } => (LightMethod::Get, Cow::from(Route::getdel(uri, key))),
            RouteInfo::GetSet { key, value } => {
                (LightMethod::Get, Cow::from(Route::getset(uri, key, value)))
            }

            RouteInfo::GetRange { key, start, end } => (
                LightMethod::Get,
                Cow::from(Route::getrange(uri, key, start, end)),
            ),
            RouteInfo::Incr { key } => (LightMethod::Get, Cow::from(Route::incr(uri, key))),
            RouteInfo::IncrBy { key, value } => {
                (LightMethod::Get, Cow::from(Route::incrby(uri, key, value)))
            }
            RouteInfo::Set { key, value } => {
                (LightMethod::Post, Cow::from(Route::set(uri, key, value)))
            }

            RouteInfo::Del { key } => (LightMethod::Get, Cow::from(Route::del(uri, key))),
        }
    }
}
