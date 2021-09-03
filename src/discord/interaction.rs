use serde::{Deserialize, Serialize, };
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::discord::command::handle_command;
use crate::error::Error;
use crate::context::Context;

#[derive(Deserialize_repr)]
#[repr(u8)]
enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
}

#[allow(dead_code)]
#[derive(Serialize_repr)]
#[repr(u8)]
pub(crate) enum InteractionResponseType {
    Pong = 1,
    Acknowledge = 2,
    ChannelMessage = 3,
    ChannelMessageWithSource = 4,
    ACKWithSource = 5,
}

// #[derive(Deserialize)]
// pub(crate) struct Member { 
//     pub(crate) name: String,
//     id: u64,
    // deaf : bool,
    // mute : bool,
    // pending: bool,
    //permissions: String,
// }

#[derive(Deserialize)]  
pub(crate) struct ApplicationCommandInteractionData {
    pub(crate) name: String,
    //id: u64,
    // application_id: u64,
    //member: Member,
    //token : String,
}

#[derive(Serialize)]
pub(crate) struct InteractionApplicationCommandCallbackData {
    pub(crate) content: String,
}

#[derive(Deserialize)]
pub(crate) struct Interaction {
    #[serde(rename = "type")]
    ty: InteractionType,
    data: Option<ApplicationCommandInteractionData>,
}

impl Interaction {
    fn data(&self) -> Result<&ApplicationCommandInteractionData, Error> {
        Ok(self
            .data
            .as_ref()
            .ok_or_else(|| Error::InvalidPayload("data not found".to_string()))?)
    }
}

#[derive(Serialize)]
pub(crate) struct InteractionResponse {
    #[serde(rename = "type")]
    pub(crate) ty: InteractionResponseType,
    pub(crate) data: Option<InteractionApplicationCommandCallbackData>,
}

impl Interaction {
    pub(crate) async fn perform(&self, context: &Context) -> Result<InteractionResponse, Error> {
        Ok(match self.ty {
            InteractionType::Ping => InteractionResponse {
                ty: InteractionResponseType::Pong,
                data: None,
            },
            InteractionType::ApplicationCommand => handle_command(context, self.data()?).await,
        })
    }
}
