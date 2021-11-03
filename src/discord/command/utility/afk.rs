use crate::discord::prelude::*;

pub(crate) async fn setafk(ctx: &CommandContext) -> CommandResult {
    let msg = if let Some(CommandOptionValue::String(msg)) = &ctx.arguments().first().map(|o| &o.value) {
        msg.trim()
    } else {
        ""
    };
    let redis = ctx.redis();
    if msg != "" {
        let key = format!("afk:{}:{}", ctx.user.clone().unwrap().id, ctx.guild_id.unwrap());

        redis.set(key.as_str(), msg).await?;
    }
    Ok(InteractionResponse::ChannelMessageWithSource(
        CallbackDataBuilder::new().build(),
    ))
}
