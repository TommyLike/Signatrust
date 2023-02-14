use super::entity::DataKey;
use crate::util::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Repository: Send + Sync {
    async fn create(&self, data_key: &DataKey) -> Result<DataKey>;
    async fn get_all(&self) -> Result<Vec<DataKey>>;
    async fn get_by_id(&self, id: i32) -> Result<DataKey>;
    async fn get_by_type_and_name(&self, key_type: String, name: String) -> Result<DataKey>;
    async fn delete_by_id(&self, id: i32) -> Result<()>;
}
