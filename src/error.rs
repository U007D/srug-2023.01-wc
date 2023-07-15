use std::error::Error as StdError;

use {clap::error::ErrorKind as ClapErrorKind, clap::Error as ClapError};
use thiserror::Error;

use crate::Error::InternalError;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum Error {
    #[error("Bad argument error: {0}")]
    BadArgument(Box<dyn StdError>),
    #[error("Word count overflowed `usize` (max: {} words)", usize::MAX)]
    CountOverflow,
    #[error("Internal error: Unexpected error encountered: {0}")]
    InternalError(Box<dyn StdError>),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

impl From<ClapError> for Error {
    fn from(err: ClapError) -> Self {
        use ClapErrorKind::*;

        match err.kind() {
            MissingRequiredArgument | UnknownArgument => Self::BadArgument(err.into()),
            _ => InternalError(err.into()),
        }
    }
}
