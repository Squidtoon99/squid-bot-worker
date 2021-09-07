use serde::{Deserialize, Serialize, };
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_json::Value;

use crate::discord::command::handle_command;
use crate::discord::component::handle_component;
use crate::error::Error;
use crate::context::Context;

#[derive(Deserialize_repr)]
#[repr(u8)]
pub(crate) enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
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
    DefferedUpdateMessage = 6,
    UpdateMessage = 7,
}

#[allow(dead_code)]
#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
pub(crate) enum CommandOptionType {
    SubCommand = 1,
    SubCommandGroup = 2,	
    String = 3,	
    Integer	= 4,	    // Any integer between -2^53 and 2^53
    Boolean	= 5,	
    User = 6,	
    Channel	= 7,	    // Includes all channel types + categories
    Role = 8,	
    Mentionable = 9,	// Includes users and roles
    Number = 10,	    // Any double between -2^53 and 2^53
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
    pub(crate) id: String,
    #[serde(rename = "type")]
    pub(crate) ty: u8,
    pub(crate) options: Vec<CommandOption>
}

#[derive(Deserialize)]
pub(crate) struct User {
    avatar: Option<String>,
    discriminator: String,
    public_flags: u16,
    username: String
}

#[derive(Deserialize)]
pub(crate) struct Member {
    user: Option<User>,
    deaf: bool,
    is_pending: bool,
    mute: bool,
    nick: Option<String>,
    pending: bool,
    permissions: Option<String>,
    premium_since: Option<String>,
    roles: Option<Vec<String>>
}

#[derive(Serialize)]
pub(crate) struct InteractionApplicationCommandCallbackData {
    pub(crate) content: String,
}

#[derive(Deserialize)]
pub(crate) struct ApplicationComponentInteractionData {
    pub(crate) custom_id: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct ComponentInteraction {
    #[serde(rename = "type")]
    ty: InteractionType,
    data: Option<ApplicationComponentInteractionData>,
}

impl ComponentInteraction {
    fn data(&self) -> Result<&ApplicationComponentInteractionData, Error> {
        Ok(self.data.as_ref().ok_or_else(|| Error::InvalidPayload("data not found".to_string()))?)
    }
}

#[derive(Deserialize, Debug)]
pub struct CommandOption {
    pub(crate) name: String,
    #[serde(rename = "type")]
    pub(crate) ty: CommandOptionType,
    pub(crate) value: Value
}

#[derive(Deserialize)]
pub(crate) struct Interaction {
    #[serde(rename = "type")]
    pub(crate) ty: InteractionType,
    data: Option<ApplicationCommandInteractionData>,
    pub(crate) guild_id: Option<String>,
    pub(crate) id: String,
    //member: Option<Member>
}

impl Interaction {
    pub fn data(&self) -> Result<&ApplicationCommandInteractionData, Error> {
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
            
            InteractionType::ApplicationCommand => handle_command(context, &self).await,
            InteractionType::Ping => InteractionResponse {
                ty: InteractionResponseType::Pong,
                data: None,
            },
            InteractionType::MessageComponent => {
                InteractionResponse {
                    ty: InteractionResponseType::Acknowledge,
                    data: None
                }
            }
        })
    }
}

impl ComponentInteraction {
    pub(crate) async fn perform(&self, context: &Context) -> Result<InteractionResponse, Error> {
        Ok(match self.ty {
            InteractionType::MessageComponent => handle_component(context, self.data()?).await,
            InteractionType::ApplicationCommand => {
                InteractionResponse {
                    ty: InteractionResponseType::Pong,
                    data: None
                }
            },
            InteractionType::Ping => {
                InteractionResponse {
                    ty: InteractionResponseType::Pong,
                    data: None, // Starting type t
                }
            }
        })
    }
}