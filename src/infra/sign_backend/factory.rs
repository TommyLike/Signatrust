use std::str::FromStr;
use crate::domain::sign_service::{SignBackend, SignBackendType};
use crate::util::error::{Result};
use std::sync::{Arc, RwLock};
use config::{Config};
use crate::infra::database::pool::DbPool;
use crate::infra::sign_backend::memory::backend::MemorySignBackend;

pub struct SignBackendFactory {}

impl SignBackendFactory {
    pub async fn new_engine(config: Arc<RwLock<Config>>, db_pool: DbPool) -> Result<Box<dyn SignBackend>> {
        let engine_type = SignBackendType::from_str(
            config.read()?.get_string("sign-backend.type")?.as_str(),
        )?;
        info!("sign backend configured with plugin {:?}", engine_type);
        match engine_type {
            SignBackendType::Memory => Ok(Box::new(MemorySignBackend::new(config, db_pool).await?)),
        }
    }
}
