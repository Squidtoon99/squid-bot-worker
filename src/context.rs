use crate::discord::handle_command;
use crate::discord::verification::verify_signature;
use crate::http::{HttpRequest, HttpResponse};
use crate::redis::client::RedisClient;
use crate::Error;
use serde::Deserialize;
use std::collections::HashMap;
use twilight_model::application::{callback::InteractionResponse, interaction::Interaction};

#[derive(Deserialize, Clone)]
pub(crate) struct Context {
    pub(crate) env: HashMap<String, String>,
    pub(crate) request: HttpRequest,
}

impl Context {
    pub fn env(&self, key: &str) -> Result<&String, Error> {
        self.env
            .get(key)
            .ok_or_else(|| Error::EnvironmentVariableNotFound(key.to_string()))
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
        let interaction = serde_json::from_str::<Interaction>(payload)?;

        let resp = match interaction {
            Interaction::Ping(_) => InteractionResponse::Pong,

            Interaction::ApplicationCommand(command) => {
                handle_command(&self, command.as_ref()).await?
            }

            _ => InteractionResponse::Pong,
        };

        serde_json::to_string(&resp).map_err(Error::JsonFailed)
    }

    pub(crate) async fn handle_http_request(&self) -> HttpResponse {
        let result = self.perform_verification();

        match result {
            Ok(()) => match self.handle_payload().await {
                Ok(response) => HttpResponse {
                    body: response,
                    status: 200,
                },
                Err(error) => HttpResponse {
                    body: error.to_string(),
                    status: 500,
                },
            },
            Err(error) => HttpResponse {
                body: error.to_string(),
                status: 500,
            },
        }
    }
}
