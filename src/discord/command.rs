mod ping;
mod amari;
mod mathsolving;

use crate::discord::interaction::{
    Interaction, InteractionResponse, InteractionResponseType, CommandOptionType
};
use crate::context::Context;


pub(crate) async fn handle_command(context: &Context, data: &Interaction) -> InteractionResponse {

    let base_data = data.data().unwrap();
    let option = &base_data.options.first().unwrap();
    let name = match  option.ty {
        CommandOptionType::SubCommand => {format!("{} {}", base_data.name.as_str(), option.value.as_str().unwrap())},
        _ => {String::from(base_data.name.as_str())}
    };

    match name.as_str() {
        "ping" => ping::ping(context, data).await,
        "amari" => amari::amari(context).await,
        "math" => mathsolving::math::math(data).await,
        _ => InteractionResponse {
            ty: InteractionResponseType::ACKWithSource,
            data: None,
        },
    }
}