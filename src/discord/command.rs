// mod amari;
mod mathsolving;
//mod messagecounting;
// mod ping;
// mod polling;
mod fun;
mod utility;

use super::prelude::*;
use crate::context::Context;

use twilight_model::application::callback::InteractionResponse;
use twilight_util::builder::CallbackDataBuilder;

pub(crate) async fn handle_command(
    context: &Context,
    interaction: &ApplicationCommand,
) -> Result<InteractionResponse> {
    let ctx = CommandContext::new(context, interaction);

    match ctx.command_name().as_str() {
        // Utility
        "ping" => utility::ping(&ctx).await,
        // "vote" => utility::vote().await,
        // "afk" => utility::afk(context, data).await,
        //&"about" => utility::about(&interaction.data).await,

        //&"links" => utility::links(&ctx).await,

        // Fun
        //&"8ball" => fun::eightball(&interaction.data).await,
        // // Mathsolving
        //&"math" => mathsolving::math(&interaction).await,

        // // Message Counting
        //&"messages" => messagecounting::messages(&ctx).await,

        // // Polling
        // "poll" => polling::poll(data).await,
        n => Ok(InteractionResponse::ChannelMessageWithSource(
            CallbackDataBuilder::new()
                .content(format!("Command **`{}`** not found", n))
                .flags(MessageFlags::EPHEMERAL)
                .build(),
        )),
    }
}
