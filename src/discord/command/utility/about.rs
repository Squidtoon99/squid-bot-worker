use crate::discord::prelude::*;

pub(crate) async fn about(_ctx: &CommandContext) -> CommandResult {
    Ok(InteractionResponse::ChannelMessageWithSource(
        CallbackDataBuilder::new()
            .embeds([EmbedBuilder::new()
                .title("Information about Friskytool")
                .description(
                    "
                        Created by `Frisky` & [`Squidtoon99`](https://squid.pink)

                        Premium:​ ​https://patreon.com/friskytool
​
                        Website:​ ​https://frisky.dev/
                        ​
                        Support Server:​ [​discord.gg/TMu242J](https://discord.gg/TMu242J)
                    ",
                )
                .build()?])
            .build(),
    ))
}
