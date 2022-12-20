use async_trait::async_trait;
use std::fmt::Error;

#[async_trait]
pub trait KMSProvider {
    async fn encode(&self, key_id: &str, content: Vec<u8>) -> Result<Vec<u8>, Error>;
    async fn decode(&self, key_id: &str, content: Vec<u8>) -> Result<Vec<u8>, Error>;
}
