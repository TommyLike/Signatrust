use crate::infra::cipher::engine::EncryptionEngine;
use crate::infra::kms::kms_provider::KMSProvider;
use crate::model::clusterkey::entity::ClusterKey;
use crate::model::datakey::entity::DataKey;
use crate::model::datakey::entity::KeyType;
use crate::model::datakey::traits::ExtendableAttributes;
use crate::util::error::Result;
use crate::util::key;
use chrono::{DateTime, Utc};
use hex;
use sqlx::FromRow;
use std::boxed::Box;
use std::convert::identity;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug, FromRow)]
pub(super) struct DataKeyDTO {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub user: String,
    pub email: String,
    pub attributes: String,
    pub key_type: String,
    pub private_key: String,
    pub public_key: String,
    pub certificate: String,
    pub create_at: DateTime<Utc>,
    pub expire_at: DateTime<Utc>,
}

impl DataKeyDTO {
    pub async fn encrypt(
        data_key: &DataKey,
        encryption_engine: Arc<Box<dyn EncryptionEngine>>,
    ) -> Result<Self> {
        Ok(Self {
            id: data_key.id,
            name: data_key.name.clone(),
            description: data_key.description.clone(),
            user: data_key.user.clone(),
            email: data_key.email.clone(),
            attributes: data_key.serialize_attributes()?,
            key_type: data_key.key_type.to_string(),
            private_key: key::encode_u8_to_hex_string(
                &encryption_engine
                    .encode(data_key.private_key.clone())
                    .await?,
            ),
            public_key: key::encode_u8_to_hex_string(
                &encryption_engine
                    .encode(data_key.public_key.clone())
                    .await?,
            ),
            certificate: key::encode_u8_to_hex_string(
                &encryption_engine
                    .encode(data_key.certificate.clone())
                    .await?,
            ),
            create_at: data_key.create_at,
            expire_at: data_key.expire_at,
        })
    }

    pub async fn decrypt(
        &self,
        encryption_engine: Arc<Box<dyn EncryptionEngine>>,
    ) -> Result<DataKey> {
        Ok(DataKey {
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone(),
            user: self.user.clone(),
            email: self.email.clone(),
            attributes: serde_json::from_str(self.attributes.as_str())?,
            key_type: KeyType::from_str(&self.key_type)?,
            private_key: encryption_engine
                .decode(key::decode_hex_string_to_u8(self.private_key.clone()))
                .await?,
            public_key: encryption_engine
                .decode(key::decode_hex_string_to_u8(self.public_key.clone()))
                .await?,
            certificate: encryption_engine
                .decode(key::decode_hex_string_to_u8(self.certificate.clone()))
                .await?,
            create_at: self.create_at,
            expire_at: self.expire_at,
        })
    }
}