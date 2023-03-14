use crate::util::error::Result;
use std::collections::HashMap;

use crate::model::datakey::entity::DataKey;
use async_trait::async_trait;

#[async_trait]
pub trait SignBackend: Send + Sync {
    async fn generate_keys(&self, data_key: &mut DataKey) -> Result<()>;
    async fn sign(&self, data_key: &DataKey, content: Vec<u8>, options: HashMap<String, String>) -> Result<Vec<u8>>;
    async fn decode_public_keys(&self, data_key: &mut DataKey) -> Result<()>;
}
