use async_trait::async_trait;
use crate::util::error::Result;

#[async_trait]
pub trait KMSProvider {
    async fn encode(&self, content: String) -> Result<String>;
    async fn decode(&self, content: String) -> Result<String>;
}
