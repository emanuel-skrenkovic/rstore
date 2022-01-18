use ::eventstore;
use std::fmt::Formatter;

use eventstore::ClientSettingsParseError;

#[derive(Debug)]
pub enum Error {
    Input { message: String },
    NotFound { message: String },
    Internal { message: String },
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Input { message: _ } => {
                write!(f, "Input Error")
            }
            Error::NotFound { message: _ } => {
                write!(f, "Not Found Error")
            }
            Error::Internal { message: _ } => {
                write!(f, "Internal Error")
            }
        }
    }
}

impl From<eventstore::ClientSettingsParseError> for Error {
    fn from(client_settings_parse_error: ClientSettingsParseError) -> Self {
        Error::Internal {
            message: format!("{:?}", client_settings_parse_error),
        }
    }
}

impl From<eventstore::Error> for Error {
    fn from(eventstore_error: eventstore::Error) -> Self {
        Error::Internal {
            message: format!("{:?}", eventstore_error),
        }
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(serde_error: serde_json::Error) -> Self {
        Error::Internal {
            message: format!("{:?}", serde_error),
        }
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(boxed_error: Box<dyn std::error::Error>) -> Self {
        Error::Internal {
            message: format!("{:?}", boxed_error.as_ref()),
        }
    }
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
