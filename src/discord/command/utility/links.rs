use crate::discord::command::prelude::*;

pub(crate) async fn links(_ctx: &CommandData) -> Result<InteractionResponse> {
    let e = EmbedBuilder::new()
                .description(
"> [Invite link (Bot Invite)](https://discord.com/oauth2/authorize?client_id=719932760604803091&scope=bot%20applications.commands&permissions=1611000937)\n\n> [Invite Link (Support server)](https://discord.gg/TMu242J)\n\n> [Click this link to vote!](https://top.gg/bot/700743797977514004/vote)")
                .build()?;

    let c = [Component::ActionRow(ActionRow {
                components: vec![Component::Button(Button {
                    custom_id: Some("abcdefg".to_string()),
                    disabled: false,
                    emoji: Some(ReactionType::Unicode {
                        name: "\u{1F517}".to_string(),
                    }),
                    label: Some("Bot Invite".to_string()),
                    style: ButtonStyle::Link,
                    url: Some("https://example.com".to_string()),
                })],
            })];

    Ok(InteractionResponse::ChannelMessageWithSource(
        CallbackDataBuilder::new()
            .embeds([e])
            .components(c)
            .build(),
    ))
}
