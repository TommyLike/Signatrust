use crate::util::error::Result;

use chrono::{DateTime, Duration, Utc};
use std::fmt::{Display, Formatter};
use std::vec::Vec;

#[derive(Debug)]
pub struct ClusterKey {
    pub id: i32,
    pub data: Vec<u8>,
    pub algorithm: String,
    pub identity: String,
    pub create_at: DateTime<Utc>,
    pub expire_at: DateTime<Utc>,
}

impl Default for ClusterKey {
    fn default() -> Self {
        ClusterKey {
            id: 0,
            data: vec![0, 0, 0, 0],
            algorithm: "".to_string(),
            identity: "".to_string(),
            create_at: Default::default(),
            expire_at: Default::default(),
        }
    }
}

impl Display for ClusterKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}, data: ******, algorithm: {}",
            self.id, self.algorithm
        )
    }
}

impl ClusterKey {
    pub fn new(data: Vec<u8>, algorithm: String, keep_in_days: i64) -> Result<Self> {
        let now = Utc::now();
        let identity = format!("{}-{}", algorithm, now.format("%d-%m-%Y"));
        Ok(ClusterKey {
            id: 0,
            data,
            algorithm,
            identity,
            create_at: now,
            expire_at: now + Duration::days(keep_in_days),
        })
    }
}
