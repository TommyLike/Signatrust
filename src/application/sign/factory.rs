use std::str::FromStr;
use crate::domain::sign_service::{SignService, SignServiceType};
use crate::util::error::{Result, Error};
use std::sync::{Arc, RwLock};
use config::{Config};
use crate::infra::database::pool::DbPool;
use crate::application::sign::memory::service::MemorySignService;

pub struct SignServiceFactory {}

impl SignServiceFactory {
    pub async fn new_engine(config: Arc<RwLock<Config>>, db_pool: DbPool) -> Result<Box<dyn SignService>> {
        let engine_type = SignServiceType::from_str(
            config.read()?.get_string("sign-backend.type")?.as_str(),
        )?;
        info!("sign backend configured with plugin {:?}", engine_type);
        match engine_type {
            SignServiceType::Memory => Ok(Box::new(MemorySignService::new(config, db_pool).await?)),
        }
    }
}
