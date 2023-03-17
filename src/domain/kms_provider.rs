use crate::util::error::{Error, Result};
use async_trait::async_trait;
use std::str::FromStr;

#[derive(Debug)]
pub enum KMSType {
    HuaweiCloud,
    Dummy,
}

impl FromStr for KMSType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "huaweicloud" => Ok(KMSType::HuaweiCloud),
            "dummy" => Ok(KMSType::Dummy),
            _ => Err(Error::UnsupportedTypeError(format!("{} kms type", s))),
        }
    }
}


#[async_trait]
pub trait KMSProvider: Send + Sync {
    async fn encode(&self, content: String) -> Result<String>;
    async fn decode(&self, content: String) -> Result<String>;
}
