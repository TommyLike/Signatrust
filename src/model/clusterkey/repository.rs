use super::entity::ClusterKey;
use crate::util::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Repository: Send + Sync {
    async fn create(&self, cluster_key: &ClusterKey) -> Result<()>;
    async fn get_latest(&self, algorithm: &str) -> Result<Option<ClusterKey>>;
    async fn get_by_id(&self, id: i32) -> Result<ClusterKey>;
    async fn delete_by_id(&self, id: i32) -> Result<()>;
}
