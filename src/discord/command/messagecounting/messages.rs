use crate::discord::prelude::*;
use serde_json::Value;
pub(crate) async fn messages(ctx: &CommandContext) -> CommandResult {
    let redis = ctx.redis();
    let messages = match redis
        .get(format!(
            "MessageCounting.{}:{}:messages",
            ctx.guild_id.unwrap(),
            ctx.user.unwrap().id
        ))
        .await?
        .result
    {
        Value::Number(n) => n.as_u64().unwrap(),
        _ => 0,
    };

    Ok(InteractionResponse::ChannelMessageWithSource(
        CallbackDataBuilder::new()
            .embeds([EmbedBuilder::new()
                .title("Messages")
                .description(format!("You have sent {} messages.", messages))
                .build()?])
            .build(),
    ))
}
