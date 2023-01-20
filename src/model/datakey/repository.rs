use super::entity::DataKey;
use crate::util::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Repository: Send + Sync {
    async fn create(&self, data_key: &DataKey) -> Result<()>;
    async fn get_by_id(&self, id: i32) -> Result<DataKey>;
    async fn get_by_name(&self, name: String) -> Result<DataKey>;
    async fn delete_by_id(&self, id: i32) -> Result<()>;
}
