use std::{
    error::Error as StdError,
    fmt::{self, Display, Error as FormatError},
    io::Error as IoError,
    num::ParseIntError,
};

use reqwest::{header::InvalidHeaderValue, Error as ReqwestError};
use serde_json::Error as JsonError;
use tracing::instrument;

use super::{HttpError, Value};

/// The common result type between most library functions.
///
/// The library exposes functions which, for a result type, exposes only one
/// type, rather than the usual 2 (`Result<T, Error>`). This is because all
/// functions that return a result return serenity's [`Error`], so this is
/// implied, and a "simpler" result is used.

/// A common error enum returned by most of the library's functionality within a
/// custom [`Result`].
///
/// The most common error types, the [`ClientError`] and [`GatewayError`]
/// enums, are both wrapped around this in the form of the [`Self::Client`] and
/// [`Self::Gateway`] variants.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// An error while decoding a payload.
    Decode(&'static str, Value),
    /// There was an error with a format.
    Format(FormatError),
    /// An [`std::io`] error.
    Io(IoError),
    /// An error from the [`serde_json`] crate.
    Json(JsonError),
    /// An error from the [`model`] module.
    ///
    /// [`model`]: crate::model
    /// An error occurred while parsing an integer.
    Num(ParseIntError),
    /// Input exceeded a limit.
    /// Providing the input and the limit that's not supposed to be exceeded.
    ///
    /// *This only exists for the [`GuildId::ban`] and [`Member::ban`] functions. For their cases,
    /// it's the "reason".*
    ///
    /// [`GuildId::ban`]: crate::model::id::GuildId::ban
    /// [`Member::ban`]: crate::model::guild::Member::ban
    ExceededLimit(String, u32),

    Http(Box<HttpError>),
}

impl From<FormatError> for Error {
    fn from(e: FormatError) -> Error {
        Error::Format(e)
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Error {
        Error::Io(e)
    }
}

impl From<JsonError> for Error {
    fn from(e: JsonError) -> Error {
        Error::Json(e)
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Error {
        Error::Num(e)
    }
}

impl From<HttpError> for Error {
    fn from(e: HttpError) -> Error {
        Error::Http(Box::new(e))
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(e: InvalidHeaderValue) -> Error {
        HttpError::InvalidHeader(e).into()
    }
}

impl From<ReqwestError> for Error {
    fn from(e: ReqwestError) -> Error {
        HttpError::Request(e).into()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Decode(msg, _) => f.write_str(msg),
            Error::ExceededLimit(..) => f.write_str("Input exceeded a limit"),
            Error::Format(inner) => fmt::Display::fmt(&inner, f),
            Error::Io(inner) => fmt::Display::fmt(&inner, f),
            Error::Json(inner) => fmt::Display::fmt(&inner, f),
            Error::Num(inner) => fmt::Display::fmt(&inner, f),
            Error::Http(inner) => fmt::Display::fmt(&inner, f),
        }
    }
}

impl StdError for Error {
    #[instrument]
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Format(inner) => Some(inner),
            Error::Io(inner) => Some(inner),
            Error::Json(inner) => Some(inner),
            Error::Num(inner) => Some(inner),

            Error::Http(inner) => Some(inner),
            _ => None,
        }
    }
}
