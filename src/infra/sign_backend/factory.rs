use std::str::FromStr;
use crate::infra::sign_backend::traits::SignBackend;
use crate::util::error::{Result, Error};
use std::sync::{Arc, RwLock};
use config::{Config};
use crate::infra::database::pool::DbPool;
use crate::infra::sign_backend::memory::engine::MemorySignBackend;

#[derive(Debug)]
enum SignBackendType {
    Memory,
}

impl FromStr for SignBackendType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "memory" => Ok(SignBackendType::Memory),
            _ => Err(Error::UnsupportedTypeError(format!("{} sign backend type", s))),
        }
    }
}

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
