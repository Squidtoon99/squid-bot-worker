use crate::discord::command::prelude::*;

pub(crate) async fn links(_ctx: &CommandData) -> Result<InteractionResponse> {
    let link_button = Button {
        custom_id: None,
        disabled: false,
        emoji: None,
        label: None,
        style: ButtonStyle::Link,
        url: None
    };

    let c = [Component::ActionRow(ActionRow {
                components: vec![
                    Component::Button(Button {
                        label: Some("Bot Invite".to_string()),
                        url: Some("https://discord.com".to_string()),
                        ..link_button.clone()
                    }),
                    Component::Button(Button {
                        label: Some("Website".to_string()),
                        url: Some("https://beta.frisky.dev".to_string()),
                        ..link_button.clone()
                    }),
                    Component::Button(Button {
                        label: Some("Vote".to_string()),
                        url: Some(format!("https://top.gg/bot/700743797977514004/vote")),
                        ..link_button.clone()
                    })
            ],
    })];

    Ok(InteractionResponse::ChannelMessageWithSource(
        CallbackDataBuilder::new().components(c).build(),
    ))
}
