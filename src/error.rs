use std::fmt::{Display, Formatter};

use {clap::error::ErrorKind as ClapErrorKind, clap::Error as ClapError};

use crate::Error::InternalError;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    BadArgument(String),
    CountOverflow,
    InternalError(String),
    IoError(std::io::Error),
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

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Error::*;

        write!(
            f,
            "{}",
            match self {
                BadArgument(s) => format!("Bad argument {s}"),
                _ => "...".into(),
            }
        )
    }
}

impl std::error::Error for Error {}
