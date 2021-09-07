use serde::Deserialize;
use std::env;
use std::collections::HashMap;
use crate::discord::interaction::{Interaction, ComponentInteraction};
use crate::discord::verification::verify_signature;
use crate::error::Error;
use crate::http::{HttpRequest, HttpResponse};
use crate::redis::client::RedisClient;

#[derive(Deserialize)]
pub(crate) struct Context {
    pub(crate) env: HashMap<String, String>,
    pub(crate) request: HttpRequest,
    #[serde(rename="type")]
    pub(crate) ty: u64
}

impl Context {
    pub fn env(&self, key: &str) -> Result<&String, Error> {
        self.env
            .get(key)
            .ok_or_else(|| Error::EnvironmentVariableNotFound(key.to_string()))
    }

    pub fn new_redis(&self) -> RedisClient {
        RedisClient::new(String::from(self.env.get("UPSTASH_URI").unwrap_or(&"".to_string())), String::from(self.env.get("UPSTASH_TOKEN").unwrap_or(&"".to_string())))
    }
    fn perform_verification(&self) -> Result<(), Error> {
        let public_key = self.env("PUBLIC_KEY")?;
        let signature = self.request.header("x-signature-ed25519")?;
        let timestamp = self.request.header("x-signature-timestamp")?;

        verify_signature(public_key, signature, timestamp, &self.request.body)
            .map_err(Error::VerificationFailed)
    }

    async fn handle_payload(&self) -> Result<String, Error> {
        let payload = &self.request.body;
        // for (key, value) in self.env.iter() {
        //     env::set_var(key, value)
        // };
        let response = match self.ty {
            3 => serde_json::from_str::<ComponentInteraction>(payload).map_err(Error::JsonFailed)?.perform(&self).await?,
            _ => { serde_json::from_str::<Interaction>(payload).map_err(Error::JsonFailed)?.perform(&self).await? }
        };
            

        serde_json::to_string(&response).map_err(Error::JsonFailed)
    }

    pub(crate) async fn handle_http_request(&self) -> HttpResponse {
        let result = self.perform_verification();
           

        match result {
            Ok(()) => {match self.handle_payload().await {
                Ok(response) => HttpResponse {body: response, status: 200},
                Err(error) => HttpResponse {body: error.to_string(), status: 500}
            } },
            Err(error) => HttpResponse {
                body: error.to_string(),
                status: 500,
            },
        }
    }
}
