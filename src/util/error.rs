use sqlx::Error as SqlxError;
use std::io::Error as IOError;
use thiserror::Error as ThisError;
use config::ConfigError as ConfigError;
use std::sync::PoisonError;
use serde_json::Error as SerdeError;
use reqwest::Error as RequestError;
use std::net::AddrParseError;
use tonic::transport::Error as TonicError;
use reqwest::header::{InvalidHeaderValue, ToStrError as StrError};
use std::num::ParseIntError;

pub type Result<T> = std::result::Result<T, Error>;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, ThisError)]
pub enum Error {
    #[error("An error occurred in database operation. {0}")]
    DatabaseError(String),
    #[error("An error occurred when loading configure file. {0}")]
    ConfigError(String),
    #[error(transparent)]
    IOError(#[from] IOError),
    #[error("unsupported type configured. {0}")]
    UnsupportedTypeError(String),
    #[error("kms invoke error. {0}")]
    KMSInvokeError(String),
    #[error("failed to serialize/deserialize. {0}")]
    SerializeError(String),
    #[error("failed to perform http request. {0}")]
    HttpRequest(String),
    #[error("failed to convert. {0}")]
    ConvertError(String)
}

impl From<SqlxError> for Error {
    fn from(sqlx_error: SqlxError) -> Self {
        match sqlx_error.as_database_error() {
            Some(db_error) => Error::DatabaseError(db_error.to_string()),
            None => {
                error!("{:?}", sqlx_error);
                Error::DatabaseError(format!("Unrecognized database error! {:?}", sqlx_error))
            }
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Error::ConfigError(error.to_string())
    }
}


impl<T> From<PoisonError<T>> for Error {
    fn from(error: PoisonError<T>) -> Self {
        Error::ConfigError(error.to_string())
    }
}

impl From<ConfigError> for Error {
    fn from(error: ConfigError) -> Self {
        Error::ConfigError(error.to_string())
    }
}

impl From<SerdeError> for Error {
    fn from(error: SerdeError) -> Self {
        Error::SerializeError(error.to_string())
    }
}

impl From<StrError> for Error {
    fn from(error: StrError) -> Self {
        Error::ConvertError(error.to_string())
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(error: InvalidHeaderValue) -> Self {
        Error::HttpRequest(error.to_string())
    }
}

impl From<RequestError> for Error {
    fn from(error: RequestError) -> Self {
        Error::HttpRequest(error.to_string())
    }
}

impl From<AddrParseError> for Error {
    fn from(error: AddrParseError) -> Self {
        Error::ConfigError(error.to_string())
    }
}

impl From<TonicError> for Error {
    fn from(error: TonicError) -> Self {
        Error::ConfigError(error.to_string())
    }
}







