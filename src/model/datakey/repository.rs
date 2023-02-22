use super::entity::DataKey;
use crate::util::error::Result;
use async_trait::async_trait;
use crate::model::datakey::entity::KeyState;

#[async_trait]
pub trait Repository: Send + Sync {
    async fn create(&self, data_key: &DataKey) -> Result<DataKey>;
    async fn get_all(&self) -> Result<Vec<DataKey>>;
    async fn get_by_id(&self, id: i32) -> Result<DataKey>;
    async fn update_state(&self, id: i32, state: KeyState) -> Result<()>;
    async fn get_enabled_key_by_type_and_name(&self, key_type: String, name: String) -> Result<DataKey>;
    async fn delete_by_id(&self, id: i32) -> Result<()>;
}
