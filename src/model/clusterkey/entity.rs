use std::vec::Vec;
use sqlx::types::chrono;
#[derive(Debug)]
pub struct ClusterKey {
    pub id: i32,
    pub data: Vec<u8>,
    pub description: String,
    pub create_at: chrono::DateTime<chrono::Utc>,
    pub expire_at: chrono::DateTime<chrono::Utc>,
}

impl Default for ClusterKey {
    fn default() -> Self {
        ClusterKey {
            id: 0,
            data: vec![0,0,0,0],
            description: "".to_string(),
            create_at: Default::default(),
            expire_at: Default::default()
        }
    }
}
