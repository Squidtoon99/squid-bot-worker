use crate::error::Error;
pub(crate) use twilight_embed_builder::*;
pub(crate) use twilight_model::application::callback::InteractionResponse;
pub(crate) use twilight_model::application::interaction::application_command::*;
pub(crate) use twilight_model::channel::message::MessageFlags;
pub(crate) use twilight_util::builder::CallbackDataBuilder;

use std::result::Result as StdResult;

pub(crate) type Result<T> = StdResult<T, Error>;
