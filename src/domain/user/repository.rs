use super::entity::User;
use crate::util::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Repository: Send + Sync {
    async fn create(&self, user: &User) -> Result<User>;
    async fn get_by_id(&self, id: i32) -> Result<User>;
    async fn get_by_email(&self, email: &str) -> Result<User>;
    async fn delete_by_id(&self, id: i32) -> Result<()>;
}
