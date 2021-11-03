use crate::discord::verification::VerificationError;
use crate::redis::Error as RedisError;
use twilight_embed_builder::EmbedError;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("Environment variable '{0}' not found.")]
    EnvironmentVariableNotFound(String),

    #[error("Header '{0}' not found.")]
    HeaderNotFound(String),

    #[error("Failed to deserialize from or serialize to JSON.")]
    JsonFailed(#[from] serde_json::Error),

    #[error("Invalid payload provided: {0}.")]
    InvalidPayload(String),

    #[error("Verification failed.")]
    VerificationFailed(VerificationError),

    #[error("Embed failed to build.")]
    EmbedFailed(EmbedError),

    #[error("Redis command failed.")]
    RedisCommandFailed(#[from] RedisError),
}

impl From<EmbedError> for Error {
    fn from(e: EmbedError) -> Error {
        Error::EmbedFailed(e)
    }
}
