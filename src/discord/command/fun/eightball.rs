use crate::discord::prelude::*;
use rand::seq::SliceRandom;

pub(crate) async fn eightball(ctx: &CommandContext<'_>) -> CommandResult {
    let options = vec![
        "It is certain.",
        "It is decidedly so.",
        "Without a doubt.",
        "Yes – definitely.",
        "You may rely on it.",
        "The stars say yes.",
        "As I see it, yes.",
        "Most likely.",
        "Outlook good.",
        "Yes.",
        "Signs point to yes.",
        "Reply hazy, try again.",
        "Ask again later.",
        "Better not tell you now.",
        "Cannot predict now.",
        "Concentrate and ask again.",
        "Don't count on it.",
        "My reply is no.",
        "My sources say no.",
        "Outlook not so good.",
        "Very doubtful.",
    ];

    let question = match &ctx.arguments().first().unwrap().value {
        CommandOptionValue::String(a) => a,
        _ => "The 8ball responds",
    };
    let mut rng = rand::thread_rng();
    let choice = options.choose(&mut rng).unwrap();
    let e = EmbedBuilder::new()
        .description(choice.to_string())
        .author(EmbedAuthorBuilder::new().name(question))
        .build()?;

    Ok(InteractionResponse::ChannelMessageWithSource(
        CallbackDataBuilder::new().embeds([e]).build(),
    ))
}
