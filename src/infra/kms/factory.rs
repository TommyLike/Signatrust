use crate::infra::kms::huaweicloud::HuaweiCloudKMS;
use crate::infra::kms::dummy::DummyKMS;
use crate::domain::kms_provider::{KMSProvider, KMSType};
use crate::util::error::{Error, Result};
use config::Value;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

pub struct KMSProviderFactory {}

impl KMSProviderFactory {
    pub fn new_provider(config: &HashMap<String, Value>) -> Result<Arc<Box<dyn KMSProvider>>> {
        let kms_type = KMSType::from_str(
            config
                .get("type")
                .unwrap_or(&Value::default())
                .to_string()
                .as_str(),
        )?;
        info!("kms provider configured with {:?}", kms_type);
        match kms_type {
            KMSType::HuaweiCloud => Ok(Arc::new(Box::new(HuaweiCloudKMS::new(config)?))),
            KMSType::Dummy => Ok(Arc::new(Box::new(DummyKMS::new(config)?))),
        }
    }
}
