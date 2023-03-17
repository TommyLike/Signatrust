use crate::infra::encryption::algorithm::aes::Aes256GcmEncryptor;
use crate::domain::encryptor::{Algorithm, Encryptor};
use crate::util::error::{Result};
use std::str::FromStr;
use std::sync::Arc;

pub struct AlgorithmFactory {}

impl AlgorithmFactory {
    pub fn new_algorithm(algo: &str) -> Result<Box<dyn Encryptor>> {
        let algorithm = Algorithm::from_str(algo)?;
        info!("encryption algorithm configured with {:?}", algorithm);
        match algorithm {
            Algorithm::Aes256GSM => Ok(Box::new(Aes256GcmEncryptor::default())),
        }
    }
}
