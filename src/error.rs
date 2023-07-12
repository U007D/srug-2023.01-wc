use {clap::error::ErrorKind as ClapErrorKind, clap::Error as ClapError};
use thiserror::Error;

use crate::Error::InternalError;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum Error {
    #[error("Bad argument error: {0}")]
    BadArgument(String),
    #[error("Word count overflowed `usize` (max: {} words)", usize::MAX)]
    CountOverflow,
    #[error("Internal error: Unexpected error encountered: {0}")]
    InternalError(String),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

impl From<ClapError> for Error {
    fn from(err: ClapError) -> Self {
        use ClapErrorKind::*;

        match err.kind() {
            MissingRequiredArgument | UnknownArgument => Self::BadArgument(err.to_string()),
            _ => InternalError(err.to_string()),
        }
    }
}
