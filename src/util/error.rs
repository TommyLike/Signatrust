use config::ConfigError;
use pgp::composed::key::SecretKeyParamsBuilderError;
use pgp::errors::Error as PGPError;
use reqwest::header::{InvalidHeaderValue, ToStrError as StrError};
use reqwest::Error as RequestError;
use serde_json::Error as SerdeError;
use sqlx::Error as SqlxError;
use std::io::Error as IOError;
use std::net::AddrParseError;
use std::num::ParseIntError;
use std::string::FromUtf8Error;
use std::sync::PoisonError;
use thiserror::Error as ThisError;
use tonic::transport::Error as TonicError;

pub type Result<T> = std::result::Result<T, Error>;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, ThisError, Clone)]
pub enum Error {
    #[error("An error occurred in database operation. {0}")]
    DatabaseError(String),
    #[error("An error occurred when loading configure file. {0}")]
    ConfigError(String),
    #[error("An error occurred when perform IO requests. {0}")]
    IOError(String),
    #[error("unsupported type configured. {0}")]
    UnsupportedTypeError(String),
    #[error("kms invoke error. {0}")]
    KMSInvokeError(String),
    #[error("failed to serialize/deserialize. {0}")]
    SerializeError(String),
    #[error("failed to perform http request. {0}")]
    HttpRequest(String),
    #[error("failed to convert. {0}")]
    ConvertError(String),
    #[error("failed to encode/decode. {0}")]
    EncodeError(String),
    #[error("failed to get cluster key. {0}")]
    ClusterError(String),
    #[error("failed to serialize/deserialize key. {0}")]
    KeyParseError(String),
    #[error("failed to sign with key {0}. {1}")]
    SignError(String, String),
    #[error("failed to perform pgp {0}")]
    PGPInvokeError(String),
    #[error("invalid parameter error {0}")]
    ParameterError(String),

    //client error
    #[error("file type not supported {0}")]
    FileNotSupportError(String),
    #[error("not any valid file found")]
    NoFileCandidateError,
    #[error("failed to split file: {0}")]
    SplitFileError(String),
    #[error("failed to remote sign file: {0}")]
    RemoteSignError(String),
    #[error("failed to assemble file: {0}")]
    AssembleFileError(String),
    #[error("failed to walk through directory: {0}")]
    WalkDirectoryError(String),
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

impl From<IOError> for Error {
    fn from(error: IOError) -> Self {
        Error::IOError(error.to_string())
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

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        Error::ConvertError(error.to_string())
    }
}

impl From<PGPError> for Error {
    fn from(error: PGPError) -> Self {
        Error::PGPInvokeError(error.to_string())
    }
}

impl From<SecretKeyParamsBuilderError> for Error {
    fn from(error: SecretKeyParamsBuilderError) -> Self {
        Error::PGPInvokeError(error.to_string())
    }
}

impl From<walkdir::Error> for Error {
    fn from(err: walkdir::Error) -> Self {
        Error::WalkDirectoryError(err.to_string())
    }
}
