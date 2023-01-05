use sqlx::FromRow;
use sqlx::types::chrono;
use crate::model::clusterkey::entity::ClusterKey;

#[derive(Debug, FromRow)]
pub(super) struct ClusterKeyDTO {
    id: i32,
    data: Vec<u8>,
    description: String,
    create_at: chrono::DateTime<chrono::Utc>,
    expire_at: chrono::DateTime<chrono::Utc>,
}

impl From<ClusterKeyDTO> for ClusterKey {
    fn from(dto: ClusterKeyDTO) -> Self {
        Self {
            id: dto.id,
            data: dto.data.clone(),
            description: dto.description.clone(),
            create_at: dto.create_at,
            expire_at: dto.expire_at
        }
    }
}
