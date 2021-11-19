mod command;
pub(crate) mod command_context;
pub(crate) mod prelude;
// pub(crate) mod interaction;
pub(crate) mod verification;
use crate::context::Context;
pub(crate) use command::handle_command;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use twilight_model::application::callback::InteractionResponse;
use twilight_model::application::interaction::Interaction;

#[post("/", data = "<payload>")]
async fn handle(ctx: Context<'_>, payload: &str) -> Result<Value, Status> {
    if let Err(e) = ctx.verify_request(String::from(payload)) {
        eprintln!("Verification failed: {:#?}", e);
        return Err(Status::Forbidden);
    }

    let interaction: Interaction = match serde_json::from_str(payload) {
        Ok(e) => e,
        Err(e) => {
            error!("{}", e);
            return Ok(json!({
                "success": false,
                "reason": "Invalid JSON"
            }));
        }
    };
    match ctx.handle_payload(interaction).await {
        Ok(response) => Ok(json!(response)),
        Err(error) => {
            eprintln!("{:#?}", error);
            Ok(json!({
                "content": "An error occurred while handling the command.",
                "embed": {
                    "title": "Error",
                    "description": format!("{:#?}", error),
                    "color": 0xFF0000,
                },
            }))
        }
    }
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("XYZ", |rocket| async { rocket.mount("/", routes![handle]) })
}
