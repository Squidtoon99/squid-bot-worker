mod ping;
mod amari;

use crate::discord::interaction::{
    ApplicationCommandInteractionData, InteractionResponse, InteractionResponseType,
};
use crate::context::Context;

pub(crate) async fn handle_command(context: &Context, data: &ApplicationCommandInteractionData) -> InteractionResponse {
    match data.name.as_str() {
        "ping" => ping::ping(context, data).await,
        "amari" => amari::amari(context).await,
        _ => InteractionResponse {
            ty: InteractionResponseType::ACKWithSource,
            data: None,
        },
    }
}