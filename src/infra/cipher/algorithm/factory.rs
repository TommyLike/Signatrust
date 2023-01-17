use crate::infra::cipher::algorithm::aes::Aes256GcmEncryptor;
use crate::infra::cipher::algorithm::traits::{Algorithm, Encryptor};
use crate::util::error::{Error, Result};
use config::Value;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use std::sync::Arc;

pub struct AlgorithmFactory {}

impl AlgorithmFactory {
    pub fn new_algorithm(algo: &str) -> Result<Arc<Box<dyn Encryptor>>> {
        let algorithm = Algorithm::from_str(algo)?;
        info!("encryption algorithm configured with {:?}", algorithm);
        match algorithm {
            Algorithm::Aes256GSM => Ok(Arc::new(Box::new(Aes256GcmEncryptor::default()))),
        }
    }
}
