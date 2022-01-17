use ::eventstore;

#[derive(Debug)]
pub enum Error {
    Input { message: String },
    Internal { message: String }
}

impl From<eventstore::Error> for Error {
    fn from(eventstore_error: eventstore::Error) -> Self {
        Error::Internal { message: format!("{:?}", eventstore_error) }
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(serde_error: serde_json::Error) -> Self {
        Error::Internal { message: format!("{:?}", serde_error) }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
