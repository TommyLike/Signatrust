use std::collections::HashMap;
use std::str::FromStr;
use config::Value;
use crate::infra::kms::huaweicloud::HuaweiCloudKMS;
use crate::infra::kms::kms_provider::KMSProvider;

#[derive(Debug)]
enum KMSType {
    HuaweiCloud,
}

impl FromStr for KMSType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "huaweicloud" => Ok(KMSType::HuaweiCloud),
            _ => Err(())
        }
    }
}


pub struct KMSProviderFactory {}

impl KMSProviderFactory {
    pub fn new_provider(config: &HashMap<String, Value>) -> Result<Box<dyn KMSProvider>, ()> {
        let kms_type = KMSType::from_str(config.get("type").unwrap().to_string().as_str()).unwrap();
        info!("kms provider configured with {:?}", kms_type);
        match kms_type {
            KMSType::HuaweiCloud => Ok(Box::new(HuaweiCloudKMS::new())),
        }
    }
}