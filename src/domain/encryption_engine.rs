use async_trait::async_trait;
use crate::util::error::Result;

#[async_trait]
pub trait EncryptionEngine: Send + Sync {
    async fn initialize(&mut self) -> Result<()>;
    async fn encode(&self, content: Vec<u8>) -> Result<Vec<u8>>;
    async fn decode(&self, content: Vec<u8>) -> Result<Vec<u8>>;
}