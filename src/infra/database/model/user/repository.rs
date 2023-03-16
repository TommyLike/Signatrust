use super::dto::UserDTO;

use crate::infra::database::pool::DbPool;
use crate::domain::user::entity::User;
use crate::domain::user::repository::Repository;
use crate::util::error::Result;
use async_trait::async_trait;
use std::boxed::Box;


#[derive(Clone)]
pub struct UserRepository {
    db_pool: DbPool,
}

impl UserRepository {
    pub fn new(db_pool: DbPool) -> Self {
        Self {
            db_pool,
        }
    }
}

#[async_trait]
impl Repository for UserRepository {

    async fn create(&self, user: &User) -> Result<User> {
        return match self.get_by_email(&user.email).await {
            Ok(existed) => {
                Ok(existed)
            }
            Err(_err) => {
                let dto = UserDTO::encrypt(user).await?;
                let record : u64 = sqlx::query("INSERT INTO user(email) VALUES (?)")
                    .bind(&dto.email)
                    .execute(&self.db_pool)
                    .await?.last_insert_id();
                self.get_by_id(record as i32).await
            }
        }
    }

    async fn get_by_id(&self, id: i32) -> Result<User> {
        let selected: UserDTO = sqlx::query_as("SELECT * FROM user WHERE id = ?")
            .bind(id)
            .fetch_one(&self.db_pool)
            .await?;
        Ok(selected.decrypt().await?)
    }

    async fn get_by_email(&self, email: &str) -> Result<User> {
        let selected: UserDTO = sqlx::query_as("SELECT * FROM user WHERE email = ?")
            .bind(email)
            .fetch_one(&self.db_pool)
            .await?;
        Ok(selected.decrypt().await?)
    }

    async fn delete_by_id(&self, id: i32) -> Result<()> {
        let _: Option<UserDTO> = sqlx::query_as("DELETE FROM user where id = ?")
            .bind(id)
            .fetch_optional(&self.db_pool)
            .await?;
        Ok(())
    }
}
