extern crate meval;
use crate::discord::prelude::*;

pub(crate) async fn math(ctx: &CommandContext<'_>) -> CommandResult {
    if let Some(CommandOptionValue::String(expression)) = &ctx.arguments().first().map(|o| &o.value)
    {
        let resp = match meval::eval_str(expression) {
            Ok(result) => format!("= `{}`", result),
            Err(e) => format!("I cannot solve that expression\n{:?}", e),
        };

        let a = EmbedAuthorBuilder::new().name(expression).build();
        let e = EmbedBuilder::new().description(resp).author(a).build()?;

        Ok(InteractionResponse::ChannelMessageWithSource(
            CallbackDataBuilder::new().embeds([e]).build(),
        ))
    } else {
        panic!("No equation passed from discord")
    }
}
