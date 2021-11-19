use crate::discord::handle_command;
use crate::discord::verification;
use crate::discord::verification::verify_signature;
use crate::Error as CrateError;
use rocket::http::{HeaderMap, Status};
use rocket::request::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::serde::json::Json;
use serde::Deserialize;
use std::collections::HashMap;
use twilight_model::application::{callback::InteractionResponse, interaction::Interaction};

#[derive(Debug, Clone, PartialEq)]
pub struct Context<'r> {
    headers: HeaderMap<'r>,
    public_key: String,
}

impl<'r> Context<'r> {
    pub fn env<S: Into<String>>(&self, key: S) -> Result<String, CrateError> {
        let x = key.into();
        std::env::var(&x)
            .ok()
            .ok_or_else(|| CrateError::EnvironmentVariableNotFound(x))
    }

    pub fn verify_request(&self, body: String) -> Result<(), verification::VerificationError> {
        let signature = self
            .headers
            .get_one("x-signature-ed25519")
            .expect("x-signature-ed25519 required");

        // sams as above but with "x-signature-timestamp"
        let timestamp = self
            .headers
            .get_one("x-signature-timestamp")
            .expect("x-signature-timestamp required");

        verify_signature(&self.public_key, signature, timestamp, &body)?;

        Ok(())
    }

    pub async fn handle_payload(&self, interaction: Interaction) -> Result<String, CrateError> {
        let resp = match interaction {
            Interaction::Ping(_) => InteractionResponse::Pong,

            Interaction::ApplicationCommand(command) => {
                handle_command(self, command.as_ref()).await?
            }

            _ => InteractionResponse::Pong,
        };
        println!("{:#?}", resp);
        serde_json::to_string(&resp).map_err(CrateError::JsonFailed)
    }
}

#[async_trait::async_trait]
impl<'r> FromRequest<'r> for Context<'r> {
    type Error = CrateError;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let public_key = std::env::var("PUBLIC_KEY")
            .expect("Expected PUBLIC_KEY environment variable to be present");

        let signature = request.headers().get_one("x-signature-ed25519");
        let timestamp = request.headers().get_one("x-signature-timestamp");

        if signature.is_none() || timestamp.is_none() {
            return Outcome::Failure((Status::BadRequest, Self::Error::MissingHeaders));
        }

        Outcome::Success(Context {
            headers: request.headers().clone(),
            public_key: public_key,
        })
    }
}
