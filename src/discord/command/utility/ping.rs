use crate::discord::command::prelude::*;

pub(crate) async fn ping(_ctx: &CommandData) -> Result<InteractionResponse> {
    let e = EmbedBuilder::new()
        .description("[Pong!](https://frisky.dev)".to_string())
        .build()?;

    Ok(InteractionResponse::ChannelMessageWithSource(
        CallbackDataBuilder::new().embeds([e]).build(),
    ))
}
