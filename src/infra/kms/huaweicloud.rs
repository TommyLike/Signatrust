use std::fmt::Error;
use async_trait::async_trait;
use crate::infra::kms::kms_provider::KMSProvider;

#[derive(Clone)]
pub struct HuaweiCloudKMS {

}

impl HuaweiCloudKMS {

    pub fn new() -> HuaweiCloudKMS {
        HuaweiCloudKMS {}
    }

}

#[async_trait]
impl KMSProvider for HuaweiCloudKMS {
    async fn encode(&self, key_id: &str, content: Vec<u8>) -> Result<Vec<u8>, Error> {
        todo!()
    }

    async fn decode(&self, key_id: &str, content: Vec<u8>) -> Result<Vec<u8>, Error> {
        todo!()
    }
}