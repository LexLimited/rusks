use core::fmt;
use std::io;


#[derive(Debug, thiserror::Error)]
pub enum Error {
    Generic,
    Reason { reason: String }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Generic => write!(f, "{}", "Generic error"),
            Error::Reason { reason } => write!(f, "{}", reason)
        }
    }
}

impl From<sled::Error> for Error {
    fn from(e: sled::Error) -> Self {
        Error::Reason { reason: e.to_string() }
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Reason { reason: e.to_string() }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Reason { reason: e.to_string() }
    }
}
