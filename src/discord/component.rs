mod ping;
use crate::discord::interaction::{
    ApplicationComponentInteractionData, InteractionResponse, InteractionResponseType,
};
use crate::context::Context;

pub(crate) async fn handle_component(_context: &Context, data: &ApplicationComponentInteractionData) -> InteractionResponse {
    match data.custom_id {
        _ => InteractionResponse {
            ty: InteractionResponseType::ACKWithSource,
            data: None,
        },
    }
}