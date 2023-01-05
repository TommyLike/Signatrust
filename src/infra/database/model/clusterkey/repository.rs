use crate::model::clusterkey::entity::ClusterKey;
use crate::model::clusterkey::repository::Repository;
use crate::infra::database::pool::DbPool;
use super::dto::ClusterKeyDTO;
use async_trait::async_trait;

pub struct ClusterKeyRepository {
    db_pool: DbPool,
}

impl ClusterKeyRepository {
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl Repository for ClusterKeyRepository {
    async fn create(&self, cluster_key: &ClusterKey) -> crate::util::error::Result<()> {
        let _ : Option<ClusterKeyDTO> = sqlx::query_as("INSERT INTO cluster_key(data, description, create_at, expire_at) VALUES (?, ?, ?, ?)")
            .bind(&cluster_key.data)
            .bind(&cluster_key.description)
            .bind(cluster_key.create_at)
            .bind(cluster_key.expire_at)
            .fetch_optional(&self.db_pool)
            .await?;
        Ok(())
    }

    async fn get_latest(&self) -> crate::util::error::Result<ClusterKey> {
        let latest: ClusterKeyDTO = sqlx::query_as("SELECT * FROM cluster_key ORDER BY id DESC LIMIT 1")
            .fetch_one(&self.db_pool)
            .await?;
        Ok(ClusterKey::from(latest))
    }

    async fn get_by_id(&self, id: i32) -> crate::util::error::Result<ClusterKey> {
        let selected: ClusterKeyDTO = sqlx::query_as("SELECT * FROM cluster_key WHERE id = ?")
            .bind(id)
            .fetch_one(&self.db_pool)
            .await?;
        Ok(ClusterKey::from(selected))
    }

    async fn delete_by_id(&self, id: i32) -> crate::util::error::Result<()> {
        let _: Option<ClusterKeyDTO> = sqlx::query_as("DELETE FROM cluster_key where id = ?")
            .bind(id)
            .fetch_optional(&self.db_pool)
            .await?;
        Ok(())
    }
}
