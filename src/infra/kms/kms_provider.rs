use crate::util::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait KMSProvider: Send + Sync {
    async fn encode(&self, content: String) -> Result<String>;
    async fn decode(&self, content: String) -> Result<String>;
}
