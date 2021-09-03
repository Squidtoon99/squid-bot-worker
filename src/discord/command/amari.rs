use crate::discord::interaction::{InteractionResponse, InteractionResponseType, InteractionApplicationCommandCallbackData};
use crate::context::Context;

pub(crate) async fn amari(context: &Context) -> InteractionResponse {
    let redis = context.new_redis();

    let map = redis.incr("test:incr").await.unwrap_or_default();
    let result = map.get("result").unwrap();

    InteractionResponse {
                ty: InteractionResponseType::ChannelMessageWithSource,
                data: Some(InteractionApplicationCommandCallbackData {
                    content: format!("Increment Score: {:?}",result)
                }),
            }
}