use crate::util::error::{Error, Result};
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub enum Algorithm {
    Aes256GSM,
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Algorithm::Aes256GSM => write!(f, "Aes256GSM"),
        }
    }
}

impl FromStr for Algorithm {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "aes256gsm" => Ok(Algorithm::Aes256GSM),
            _ => Err(Error::UnsupportedTypeError(format!(
                "{} invalid encryption algorithm type",
                s
            ))),
        }
    }
}

pub trait Encryptor: Send + Sync {
    fn generate_key(&self) -> Vec<u8>;
    fn algorithm(&self) -> Algorithm;
    fn encrypt(&self, key: Vec<u8>, content: Vec<u8>) -> Result<Vec<u8>>;
    fn decrypt(&self, key: Vec<u8>, content: Vec<u8>) -> Result<Vec<u8>>;
}
