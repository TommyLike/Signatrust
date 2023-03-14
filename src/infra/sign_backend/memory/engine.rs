use std::collections::HashMap;
use crate::model::datakey::entity::KeyType;
use super::super::traits::SignBackend;
use std::sync::Arc;
use config::Value;
use config::Config;
use std::sync::RwLock;
use crate::model::clusterkey::repository::Repository as ClusterKeyRepository;
use crate::infra::database::model::clusterkey::repository;
use crate::infra::database::pool::{DbPool, get_db_pool};
use crate::infra::kms::factory;
use crate::infra::encryption::engine::{EncryptionEngineWithClusterKey, EncryptionEngine};
use crate::infra::sign_backend::sec_key::SecKey;
use crate::infra::sign_plugin::signers::Signers;
use crate::model::datakey::entity::DataKey;
use crate::util::error::Result;
use async_trait::async_trait;


/// Memory Sign Backend will perform all sensitive operations directly in host memory.
pub struct MemorySignBackend {
    server_config: Arc<RwLock<Config>>,
    engine: Arc<Box<dyn EncryptionEngine>>
}

impl MemorySignBackend {
    /// initialize process
    /// 1. initialize the kms provider
    /// 2. initialize the cluster repo
    /// 2. initialize the encryption engine including the cluster key
    /// 3. initialize the signing plugins
    pub async fn new(server_config: Arc<RwLock<Config>>, db_pool: DbPool) -> crate::util::error::Result<MemorySignBackend> {
        //initialize the kms backend
        let kms_provider = factory::KMSProviderFactory::new_provider(
            &server_config.read()?.get_table("memory.kms-provider")?
        )?;
        let repository =
            repository::ClusterKeyRepository::new(db_pool);
        let engine_config = server_config.read()?.get_table("memory.encryption-engine")?;
        let mut engine = EncryptionEngineWithClusterKey::new(
            Arc::new(Box::new(repository.clone())),
            &engine_config,
            kms_provider
        )?;
        engine.initialize().await?;

        Ok(MemorySignBackend {
            server_config,
            engine: Arc::new(Box::new((engine))),
        })
    }
}

#[async_trait]
impl SignBackend for MemorySignBackend {
    async fn generate_keys(&self, data_key: &mut DataKey) -> Result<()> {
        let (private_key, public_key, certificate) = Signers::generate_keys(&data_key.key_type, &data_key.attributes)?;
        data_key.private_key = self.engine.encode(private_key).await?;
        data_key.public_key = self.engine.encode(public_key).await?;
        data_key.certificate = self.engine.encode(certificate).await?;
        Ok(())
    }

    async fn sign(&self, data_key: &DataKey, content: Vec<u8>, options: HashMap<String, String>) -> Result<Vec<u8>> {
        let sec_key = SecKey::load(data_key, &self.engine).await?;
        Signers::load_from_data_key(&data_key.key_type, &sec_key)?.sign(content, options)
    }

    async fn decode_public_keys(&self, data_key: &mut DataKey) -> Result<()> {
        data_key.public_key = self.engine.decode(data_key.public_key.clone()).await?;
        data_key.certificate = self.engine.decode(data_key.certificate.clone()).await?;
        Ok(())
    }
}