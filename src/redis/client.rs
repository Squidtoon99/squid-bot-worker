use reqwest::{Client, Response as ReqwestResponse};
// use reqwest::{
//     header::{HeaderMap as Headers, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
//     StatusCode,
//     Url,
// };
use std::{fmt, sync::Arc};

use super::{request::Request, routing::RouteInfo, Error, HttpError, Result, Value};

// pub(crate) struct RedisBuilder<'a> {
//     token: Option<String>,
//     client: Option<Arc<Client>>,
// }
use serde::de::DeserializeOwned;
use serde::Deserialize;
// use serde_json::json;
// use std::env;

// impl<'a> RedisBuilder<'a> {
//     fn _new() -> Self {
//         Self {
//             token: None,
//             client: None,
//         }
//     }

//     pub fn new(token: impl AsRef<str>, url : impl AsRef<str>) -> Self {
//         Self::_new().token(token);
//     }

//     pub fn token(mut self, token: impl AsRef<str>) -> Self {
//         let token = token.as_ref().trim();

//         let token = if token.starts_with("Bearer ") { token.to_string() } else {format!("Bearer {}", token) };

//         self.token = Some(token.clone());

//         self
//     }

//     pub fn client(mut self, client: Arc<Client>) -> Self {
//         self.client = Some(client);

//         self
//     }
// }

#[derive(Deserialize, Debug)]
pub(crate) struct UpstashResponse {
    pub(crate) result: Value,
}

pub(crate) struct RedisClient {
    pub(crate) client: Arc<Client>,
    pub uri: String,
    pub token: String,
}

impl fmt::Debug for RedisClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RedisClient")
            .field("token", &self.token)
            .field("client", &self.client)
            .finish()
    }
}

#[allow(dead_code)]
impl RedisClient {
    pub fn new(uri: String, token: String) -> Self {
        let built = Client::builder()
            .build()
            .expect("Cannot build Reqwest::Client.");
        let client = Arc::new(built);

        RedisClient { client, uri, token }
    }

    pub async fn get(&self, key: &str) -> Result<UpstashResponse> {
        self.fire(Request {
            body: None,
            headers: None,
            route: RouteInfo::Get { key },
        })
        .await
    }

    pub async fn set(&self, key: &str, value: &str) -> Result<UpstashResponse> {
        self.fire(Request {
            body: None,
            headers: None,
            route: RouteInfo::Set { key, value },
        })
        .await
    }

    pub async fn incr(&self, key: &str) -> Result<UpstashResponse> {
        self.fire(Request {
            body: None,
            headers: None,
            route: RouteInfo::Incr { key },
        })
        .await
    }

    pub async fn del(&self, key: &str) -> Result<UpstashResponse> {
        self.fire(Request {
            body: None,
            headers: None,
            route: RouteInfo::Del { key },
        })
        .await
    }

    pub async fn request(&self, req: Request<'_>) -> Result<ReqwestResponse> {
        let request = req.build(&self.client, &self.uri, &self.token)?.build()?;
        let response = self.client.execute(request).await?;

        if response.status().is_success() {
            Ok(response)
        } else {
            Err(Error::Http(Box::new(
                HttpError::from_response(response).await,
            )))
        }
    }

    pub async fn fire<T: DeserializeOwned>(&self, req: Request<'_>) -> Result<T> {
        let response = self.request(req).await?;

        response.json::<T>().await.map_err(From::from)
    }
}
