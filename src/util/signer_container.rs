use std::collections::HashMap;
use crate::infra::sign::traits::SignPlugins;
use std::sync::{Arc};
use tokio::sync::RwLock;
use crate::util::error::Result;
use crate::infra::database::model::datakey::repository::EncryptedDataKeyRepository;
use crate::model::datakey::repository::Repository;
use crate::infra::sign::signers::Signers;

pub struct SignerContainer {
    repository: Arc<EncryptedDataKeyRepository>,
    containers: Arc<RwLock<HashMap<String, Arc<Box<dyn SignPlugins>>>>>,
}

impl SignerContainer {
    pub fn new(repository: Arc<EncryptedDataKeyRepository>) -> SignerContainer {
        Self {
            repository,
            containers: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    pub async fn get_signer(&self, key_type: String, key_name: String) -> Result<Arc<Box<dyn SignPlugins>>> {
        let identity = self.get_identity(&key_type, &key_name);
        if let Some(signer) = self.containers.read().await.get(&identity) {
            return Ok(signer.clone())
        }
        let datakey = self.repository.get_by_type_and_name(key_type, key_name).await?;
        let new = Signers::load_from_data_key(&datakey)?;
        self.containers.write().await.insert(identity, new.clone());
        Ok(new)
    }

    fn get_identity(&self, key_type: &str, key_name: &str) -> String {
        format!("{}-{}",key_type, key_name)
    }
}