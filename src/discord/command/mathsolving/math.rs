use crate::discord::interaction::{
    InteractionApplicationCommandCallbackData, InteractionResponse, InteractionResponseType, Interaction};
extern crate meval;

pub(crate) async fn math(data: &Interaction) -> InteractionResponse {
    let interaction_data = data.data().unwrap();
    let options = &interaction_data.options;
    let expression = &options.first().unwrap().value.as_str().unwrap();
    let result = match meval::eval_str(expression) {
        Ok(result) => result.to_string(),
        Err(_) => "I cannot solve that expression".to_string()
    };

    InteractionResponse {
                ty: InteractionResponseType::ChannelMessageWithSource,
                data: Some(InteractionApplicationCommandCallbackData {
                    content: format!("{}", result),
                }),
            }
}
