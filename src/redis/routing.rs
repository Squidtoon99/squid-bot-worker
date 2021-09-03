use std::{
    env,
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
    pub fn get(key: &str) -> String {
        api(format!("/get/{}", key))
    }

    pub fn set(key: &str, value: &str) -> String {
        api(format!("/set/{}/{}", key, value))
    }

    pub fn incr(key: &str) -> String {
        api(format!("/incr/{}", key))
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
    pub fn deconstruct(&self) -> (LightMethod, Cow<'_, str>){
        match *self {
            RouteInfo::Set { key, value } => (
                LightMethod::Post,
                Cow::from(Route::set(key, value))
            ),
            RouteInfo::Get { key } => (
                LightMethod::Get,
                Cow::from(Route::get(key))
            ),
            RouteInfo::Incr { key } => (
                LightMethod::Get,
                Cow::from(Route::incr(key))
            )
        }
    }
}