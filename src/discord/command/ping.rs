use crate::discord::interaction::{
    InteractionApplicationCommandCallbackData, InteractionResponse, InteractionResponseType, ApplicationCommandInteractionData};
use crate::context::Context;
use crate::redis::client::RedisClient;

pub(crate) async fn ping(context: &Context, data: &ApplicationCommandInteractionData) -> InteractionResponse {
    
    let redis = context.new_redis();
    //redis.set("key", "value").await.unwrap();
    redis.set("key", "value").await.unwrap();//.unwrap_or(String::from("default"));
    let value = redis.get("key").await.unwrap_or_default();
    let result: String;
    if let Some(x) = value.get("result") {
        result = x.to_string();
    } else {
        result = "Uh oh error :(".to_string();
    }

    InteractionResponse {
                ty: InteractionResponseType::ChannelMessageWithSource,
                data: Some(InteractionApplicationCommandCallbackData {
                    content: format!("[Pong!](https://squid.pink/bot)\nvalue: {:?}",result)
                }),
            }
    //let value = "";
   
}
