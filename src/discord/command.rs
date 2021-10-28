// mod amari;
mod mathsolving;
// mod messagecounting;
// mod ping;
// mod polling;
mod fun;
mod utility;

mod prelude;

use crate::context::Context;
use prelude::*;

use twilight_model::application::callback::InteractionResponse;
use twilight_util::builder::CallbackDataBuilder;

pub(crate) async fn handle_command(
    _context: &Context,
    interaction: &ApplicationCommand,
) -> Result<InteractionResponse> {
    match interaction.data.name.as_str() {
        // Utility
        "ping" => utility::ping(&interaction.data).await,
        // "vote" => utility::vote().await,
        // "afk" => utility::afk(context, data).await,
        // "about" => utility::about().await,
        // "links" => utility::links().await,

        // Fun
        "8ball" => fun::eightball(&interaction.data).await,
        // // Mathsolving
        "math" => mathsolving::math(&interaction).await,

        // // Message Counting
        // "messages" => messagecounting::messages(context, data).await,

        // // Polling
        // "poll" => polling::poll(data).await,
        n => Ok(InteractionResponse::ChannelMessageWithSource(
            CallbackDataBuilder::new()
                .content(format!("Command {} not found", n))
                .build(),
        )),
    }
}
