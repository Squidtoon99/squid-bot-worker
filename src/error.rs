use crate::discord::verification::VerificationError;
use twilight_embed_builder::EmbedError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Environment variable '{0}' not found.")]
    EnvironmentVariableNotFound(String),

    #[error("Missing the required headers")]
    MissingHeaders,

    #[error("Failed to deserialize from or serialize to JSON.")]
    JsonFailed(#[from] serde_json::Error),

    #[error("Invalid payload provided: {0}.")]
    InvalidPayload(String),

    #[error("Verification failed.")]
    VerificationFailed(VerificationError),

    #[error("Embed failed to build.")]
    EmbedFailed(EmbedError),
}

impl From<EmbedError> for Error {
    fn from(e: EmbedError) -> Error {
        Error::EmbedFailed(e)
    }
}
