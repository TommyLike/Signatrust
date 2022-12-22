use std::collections::HashMap;
use std::str::FromStr;
use config::Value;
use crate::infra::kms::huaweicloud::HuaweiCloudKMS;
use crate::infra::kms::kms_provider::KMSProvider;
use crate::util::error::{Result, Error};
use std::sync::Arc;

#[derive(Debug)]
enum KMSType {
    HuaweiCloud,
}

impl FromStr for KMSType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "huaweicloud" => Ok(KMSType::HuaweiCloud),
            _ => Err(Error::UnsupportedTypeError(format!("{} kms type", s)))
        }
    }
}


pub struct KMSProviderFactory {}

impl KMSProviderFactory {
    pub fn new_provider(config: &HashMap<String, Value>) -> Result<Arc<Box<dyn KMSProvider>>> {
        let kms_type = KMSType::from_str(config.get("type").unwrap_or(&Value::default()).to_string().as_str())?;
        info!("kms provider configured with {:?}", kms_type);
        match kms_type {
            KMSType::HuaweiCloud => Ok(Arc::new(Box::new(HuaweiCloudKMS::new(config)?))),
        }
    }
}