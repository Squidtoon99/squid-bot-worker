use crate::discord::interaction::{
    InteractionApplicationCommandCallbackData, InteractionResponse, InteractionResponseType, Interaction};
use crate::context::Context;

#[allow(unused_variables)]
pub(crate) async fn ping(context: &Context, data: &Interaction) -> InteractionResponse {
    
    let redis = context.new_redis();
    redis.set("key", "success").await.unwrap();
    let value = redis.get("key").await.unwrap();

    InteractionResponse {
                ty: InteractionResponseType::ChannelMessageWithSource,
                data: Some(InteractionApplicationCommandCallbackData {
                    content: format!("[Pong!](https://squid.pink/bot)\nDB Test: {}", value.result),
                }),
            }
}
