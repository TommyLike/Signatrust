use crate::infra::database::pool::DbPool;
use crate::domain::token::entity::{Token};
use crate::domain::token::repository::Repository;
use crate::util::error::Result;
use async_trait::async_trait;
use std::boxed::Box;

use crate::infra::database::model::token::dto::TokenDTO;


#[derive(Clone)]
pub struct TokenRepository {
    db_pool: DbPool,
}

impl TokenRepository {
    pub fn new(db_pool: DbPool) -> Self {
        Self {
            db_pool,
        }
    }
}

#[async_trait]
impl Repository for TokenRepository {

    async fn create(&self, token: &Token) -> Result<Token> {
        let dto = TokenDTO::encrypt(token).await?;
        let record : u64 = sqlx::query("INSERT INTO token(user_id, token, expire_at) VALUES (?, ?, ?)")
            .bind(&dto.user_id)
            .bind(&dto.token)
            .bind(&dto.expire_at)
            .execute(&self.db_pool)
            .await?.last_insert_id();
        self.get_token_by_id(record as i32).await
    }

    async fn get_token_by_id(&self, id: i32) -> Result<Token> {
        let selected: TokenDTO = sqlx::query_as("SELECT * FROM token WHERE id = ?")
            .bind(id)
            .fetch_one(&self.db_pool)
            .await?;
        Ok(selected.decrypt().await?)
    }

    async fn get_token_by_value(&self, token: &str) -> Result<Token> {
        let selected: TokenDTO = sqlx::query_as("SELECT * FROM token WHERE token = ?")
            .bind(token)
            .fetch_one(&self.db_pool)
            .await?;
        Ok(selected.decrypt().await?)
    }

    async fn delete_by_id(&self, id: i32) -> Result<()> {
        let _: Option<TokenDTO> = sqlx::query_as("DELETE FROM token where id = ?")
            .bind(id)
            .fetch_optional(&self.db_pool)
            .await?;
        Ok(())
    }

    async fn get_token_by_user_id(&self, id: i32) -> Result<Vec<Token>> {
        let dtos: Vec<TokenDTO> = sqlx::query_as("SELECT * FROM token WHERE user_id = ?")
            .bind(id)
            .fetch_all(&self.db_pool)
            .await?;
        let mut results = vec![];
        for dto in dtos.into_iter() {
            results.push(dto.decrypt().await?);
        }
        Ok(results)
    }
}
