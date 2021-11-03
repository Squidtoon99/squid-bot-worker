extern crate meval;
use crate::discord::prelude::*;

pub(crate) async fn math(ctx: &CommandContext) -> CommandResult {
    if let CommandOptionValue::String(expression) = &ctx.arguments().first().unwrap().value {
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
