mod ping;

use crate::discord::interaction::{
    ApplicationCommandInteractionData, InteractionResponse, InteractionResponseType,
};

pub(crate) fn handle_command(data: &ApplicationCommandInteractionData) -> InteractionResponse {
    match data.name.as_str() {
        "ping" => ping::ping(),
        _ => InteractionResponse {
            ty: InteractionResponseType::ACKWithSource,
            data: None,
        },
    }
}
