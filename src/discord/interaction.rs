use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::context::Context;
use crate::discord::command::handle_command;
use crate::error::Error;
use twilight_model::application::callback::CallbackData;
use twilight_model::user::User;
use twilight_model::{
    guild::PartialMember,
    application::interaction::{InteractionType, application_command::CommandData},
    id::{ApplicationId, ChannelId, GuildId, InteractionId}
};

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



// #[derive(Serialize)]
// pub(crate) struct InteractionApplicationCommandCallbackData {
//     pub(crate) embeds: Option<Vec<Embed>>,
//     pub(crate) components: Option<Vec<InteractionApplicationComponentCallbackData>>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub(crate) flags: Option<InteractionCallbackDataFlags>,
// }

// #[allow(dead_code)]
// impl InteractionApplicationComponentCallbackData {
//     pub(crate) fn action_row(components: Vec<Self>) -> Self {
//         Self {
//             ty: InteractionApplicationComponentCallbackDataType::ActionRow,
//             components: components,

//             // useless but necessary because I'm stupid
//             custom_id: None,
//             label: None,
//             style: None,
//             url: None,
//         }
//     }

//     pub(crate) fn default_button(custom_id: String, label: String, style: ButtonStyle) -> Self {
//         Self {
//             ty: InteractionApplicationComponentCallbackDataType::Button,
//             components: Vec::new(),

//             custom_id: Some(custom_id),
//             label: Some(label),
//             style: Some(style),

//             url: None,
//         }
//     }

//     pub(crate) fn link_button(url: String, label: String) -> Self {
//         Self {
//             ty: InteractionApplicationComponentCallbackDataType::Button,
//             components: Vec::new(),
//             custom_id: None,
//             label: Some(label),
//             style: Some(ButtonStyle::Link),
//             url: Some(url),
//         }
//     }
// }

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub(crate) struct Interaction {
    #[serde(rename = "type")]
    pub(crate) ty: InteractionType,
    pub application_id: ApplicationId,
    pub channel_id: ChannelId,
    pub data: CommandData,
    pub guild_id: Option<GuildId>,
    pub id: InteractionId,
    pub kind: InteractionType,
    pub member: Option<PartialMember>,
    pub token: String,
    pub user: Option<User>,
}

#[derive(Serialize)]
pub(crate) struct InteractionResponse {
    #[serde(rename = "type")]
    pub(crate) ty: InteractionResponseType,
    pub(crate) data: CallbackData,
}

impl Interaction {
    pub(crate) async fn perform(&self, context: &Context) -> Result<InteractionResponse, Error> {
        handle_command(context, &self).await
    }
}