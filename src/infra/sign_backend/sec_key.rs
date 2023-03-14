use secstr::SecVec;
use crate::infra::encryption::engine::EncryptionEngine;
use crate::model::datakey::entity::DataKey;
use crate::model::datakey::traits::Identity;
use crate::util::error::Result;

use std::sync::Arc;
use crate::infra::kms::kms_provider::KMSProvider;
use crate::model::clusterkey::entity::ClusterKey;
use crate::util::key;
use std::fmt::{Display, Formatter};

pub struct SecKey {
    pub private_key: SecVec<u8>,
    pub public_key: SecVec<u8>,
    pub certificate: SecVec<u8>,
    pub identity: String
}

impl SecKey {
    pub async fn load(data_key: &DataKey, engine: &Arc<Box<dyn EncryptionEngine>>) -> Result<SecKey> {
        Ok(Self {
            private_key: SecVec::new(engine.decode(data_key.private_key.clone()).await?),
            public_key: SecVec::new(engine.decode(data_key.public_key.clone()).await?),
            certificate: SecVec::new(engine.decode(data_key.certificate.clone()).await?),
            identity: data_key.get_identity(),
        })
    }
}

pub struct SecClusterKey {
    pub id: i32,
    pub data: SecVec<u8>,
    pub algorithm: String,
    pub identity: String,
}

impl Default for SecClusterKey {

    fn default() -> Self {
        SecClusterKey {
            id: 0,
            data: SecVec::new(vec![0, 0, 0, 0]),
            algorithm: "".to_string(),
            identity: "".to_string(),
        }
    }
}


impl SecClusterKey {
    pub async fn load(cluster_key: ClusterKey, kms_provider: &Arc<Box<dyn KMSProvider>>) -> Result<SecClusterKey> {
        Ok(Self {
            id: cluster_key.id,
            data: SecVec::new(key::decode_hex_string_to_u8(
                &kms_provider
                    .decode(String::from_utf8(cluster_key.data)?)
                    .await?,
            )),
            identity: cluster_key.identity,
            algorithm: cluster_key.algorithm,
        })
    }
}

impl Display for SecClusterKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}, data: ******, algorithm: {}",
            self.id, self.algorithm
        )
    }
}