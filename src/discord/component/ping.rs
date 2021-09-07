use crate::discord::interaction::{
    InteractionApplicationCommandCallbackData, InteractionResponse, InteractionResponseType, ApplicationCommandInteractionData};
use crate::context::Context;

pub(crate) async fn ping(context: &Context, data: &ApplicationCommandInteractionData) -> InteractionResponse {
    InteractionResponse {
                ty: InteractionResponseType::ChannelMessageWithSource,
                data: Some(InteractionApplicationCommandCallbackData {
                    content: "[Pong!](https://squid.pink/bot)".to_string()
                }),
            }
}
