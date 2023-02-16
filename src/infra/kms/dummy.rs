use crate::infra::kms::kms_provider::KMSProvider;
use crate::util::error::{Error, Result};
use config::Value;
use std::collections::HashMap;
use async_trait::async_trait;

pub struct DummyKMS {
}

impl DummyKMS {
    pub fn new(config: &HashMap<String, Value>) -> Result<DummyKMS> {
        Ok(DummyKMS {})
    }
}

#[async_trait]
impl KMSProvider for DummyKMS {
    async fn encode(&self, content: String) -> Result<String> {
        warn!("dummy kms used for encoding, please don't use it in production environment");
        Ok(content)
    }

    async fn decode(&self, content: String) -> Result<String> {
        warn!("dummy kms used for decoding, please don't use it in production environment");
        Ok(content)
    }
}
