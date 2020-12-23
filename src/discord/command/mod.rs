use crate::discord::interaction::{
    ApplicationCommandInteractionData, InteractionResponse, InteractionResponseType,
};

mod hello;

pub(crate) fn handle_command(data: &ApplicationCommandInteractionData) -> InteractionResponse {
    match data.name.as_str() {
        "hello" => hello::hello(),
        _ => InteractionResponse {
            ty: InteractionResponseType::ACKWithSource,
            data: None,
        },
    }
}
