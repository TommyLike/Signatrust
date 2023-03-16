use std::collections::HashMap;
use std::sync::{Arc};
use tokio::sync::RwLock;
use crate::util::error::Result;
use crate::domain::datakey::repository::Repository;

use crate::domain::datakey::entity::DataKey;

pub struct DataKeyContainer<R>
where
    R: Repository
{
    repository: R,
    containers: Arc<RwLock<HashMap<String, DataKey>>>,
}

impl<R> DataKeyContainer<R>
where
    R: Repository
{
    pub fn new(repository: R) -> Self {
        Self {
            repository,
            containers: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    pub async fn get_data_key(&self, key_type: String, key_name: String) -> Result<DataKey> {
        let identity = self.get_identity(&key_type, &key_name);
        if let Some(dk) = self.containers.read().await.get(&identity) {
            return Ok((*dk).clone())
        }
        let data_key = self.repository.get_enabled_key_by_type_and_name(key_type, key_name).await?;
        self.containers.write().await.insert(identity, data_key.clone());
        Ok(data_key)
    }

    fn get_identity(&self, key_type: &str, key_name: &str) -> String {
        format!("{}-{}",key_type, key_name)
    }
}