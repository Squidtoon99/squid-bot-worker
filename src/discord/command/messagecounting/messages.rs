use crate::discord::prelude::*;
use serde_json::Value;

pub(crate) async fn messages(ctx: &CommandContext) -> CommandResult {
    let redis = ctx.redis();
    let user = if let Some(CommandOptionValue::User(u)) = &ctx.arguments().first().map(|o| &o.value)
    {
        u
    } else {
        ctx.member
            .expect("Expected member passed with command context")
            .user
            .expect("Expected user associated with member object")
            .id;
    };
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
                .description(format!("<@{}> has `{}` messages.", messages))
                .build()?])
            .build(),
    ))
}
