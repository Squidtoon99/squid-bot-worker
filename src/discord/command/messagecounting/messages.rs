use crate::discord::interaction::{InteractionResponse, InteractionResponseType, InteractionApplicationCommandCallbackData, Interaction};
use crate::context::Context;

pub(crate) async fn messages(context: &Context, data: &Interaction) -> Result<InteractionResponse> {
    let redis = context.new_redis();
    let user_id = 
    //messages = redis.get(format!("MessageCounting.{}:{}:messages"))
}