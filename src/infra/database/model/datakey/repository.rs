use super::dto::DataKeyDTO;
use crate::infra::cipher::algorithm::traits::Algorithm;
use crate::infra::cipher::engine::EncryptionEngine;
use crate::infra::database::pool::DbPool;
use crate::infra::kms::kms_provider::KMSProvider;
use crate::model::datakey::entity::DataKey;
use crate::model::datakey::repository::Repository;
use crate::util::error::{Error, Result};
use async_trait::async_trait;
use std::boxed::Box;
use std::sync::Arc;
use actix_web::web::Data;

#[derive(Clone)]
pub struct EncryptedDataKeyRepository {
    db_pool: DbPool,
    encryption_engine: Arc<Box<dyn EncryptionEngine>>,
}

impl EncryptedDataKeyRepository {
    pub fn new(db_pool: DbPool, encryption_engine: Arc<Box<dyn EncryptionEngine>>) -> Self {
        Self {
            db_pool,
            encryption_engine,
        }
    }
}

#[async_trait]
impl Repository for EncryptedDataKeyRepository {
    async fn create(&self, data_key: &DataKey) -> Result<DataKey> {
        let dto = DataKeyDTO::encrypt(data_key, self.encryption_engine.clone()).await?;
        let record : u64 = sqlx::query("INSERT INTO data_key(name, description, user, email, attributes, key_type, private_key, public_key, certificate, create_at, expire_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(&dto.name)
            .bind(&dto.description)
            .bind(&dto.user)
            .bind(dto.email)
            .bind(dto.attributes)
            .bind(dto.key_type)
            .bind(dto.private_key)
            .bind(dto.public_key)
            .bind(dto.certificate)
            .bind(dto.create_at)
            .bind(dto.expire_at)
            .execute(&self.db_pool)
            .await?.last_insert_id();
        return self.get_by_id(record as i32).await
    }

    async fn get_all(&self) -> Result<Vec<DataKey>> {
        let dtos: Vec<DataKeyDTO> = sqlx::query_as("SELECT * FROM data_key")
            .fetch_all(&self.db_pool)
            .await?;
        let mut results = vec![];
        for dto in dtos.into_iter() {
            results.push(dto.decrypt(self.encryption_engine.clone()).await?);
        }
        Ok(results)
    }

    async fn get_by_id(&self, id: i32) -> Result<DataKey> {
        let dto: DataKeyDTO = sqlx::query_as("SELECT * FROM data_key WHERE id = ?")
            .bind(id)
            .fetch_one(&self.db_pool)
            .await?;
        Ok(dto.decrypt(self.encryption_engine.clone()).await?)
    }

    async fn get_by_type_and_name(&self, key_type: String, name: String) -> Result<DataKey> {
        let dto: DataKeyDTO = sqlx::query_as("SELECT * FROM data_key WHERE name = ? AND key_type = ?")
            .bind(name)
            .bind(key_type)
            .fetch_one(&self.db_pool)
            .await?;
        Ok(dto.decrypt(self.encryption_engine.clone()).await?)
    }

    async fn delete_by_id(&self, id: i32) -> Result<()> {
        let _: Option<DataKeyDTO> = sqlx::query_as("DELETE FROM data_key WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.db_pool)
            .await?;
        Ok(())
    }
}
