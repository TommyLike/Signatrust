use sqlx::Error as SqlxError;
use std::io::Error as IOError;
use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("An error occurred in database operation. {0}")]
    DatabaseError(String),
    #[error("An error occurred when parsing configure file. {0}")]
    ConfigError(String),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

impl From<SqlxError> for Error {
    fn from(sqlx_error: SqlxError) -> Self {
        match sqlx_error.as_database_error() {
            Some(db_error) => Error::DatabaseError(db_error.to_string()),
            None => {
                error!("{:?}", sqlx_error);
                Error::DatabaseError(String::from("Unrecognized database error!"))
            }
        }
    }
}

