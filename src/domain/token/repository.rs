use super::entity::Token;
use crate::util::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Repository: Send + Sync {
    async fn create(&self, user: &Token) -> Result<Token>;
    async fn get_token_by_id(&self, id: i32) -> Result<Token>;
    async fn get_token_by_value(&self, token:  &str) -> Result<Token>;
    async fn delete_by_id(&self, id: i32) -> Result<()>;
    async fn get_token_by_user_id(&self, id: i32) -> Result<Vec<Token>>;
}
