use crate::domain::user::entity::User;
use crate::domain::token::entity::Token;
use crate::domain::user::repository::Repository as UserRepository;
use crate::domain::token::repository::Repository as TokenRepository;
use crate::util::error::{Result};
use async_trait::async_trait;
use crate::presentation::handler::control::model::user::dto::UserIdentity;

#[async_trait]
pub trait UserService: Send + Sync{
    async fn save(&self, u: &User) -> Result<User>;
    async fn get_token(&self, u: &UserIdentity) -> Result<Vec<Token>>;
    async fn generate_token(&self, u: &UserIdentity) -> Result<Token>;
}



pub struct DBUserService<R, T>
where
    R: UserRepository,
    T: TokenRepository
{
    user_repository: R,
    token_repository: T
}

impl<R, T> DBUserService<R, T>
    where
        R: UserRepository,
        T: TokenRepository
{
    pub fn new(user_repository: R, token_repository: T) -> Self {
        Self {
            user_repository,
            token_repository
        }
    }
}

#[async_trait]
impl<R, T> UserService for DBUserService<R, T>
where
    R: UserRepository,
    T: TokenRepository
{
    async fn save(&self, u: &User) -> Result<User> {
        return self.user_repository.create(u).await
    }

    async fn get_token(&self, user: &UserIdentity) -> Result<Vec<Token>> {
        return self.token_repository.get_token_by_user_id(user.id).await
    }

    async fn generate_token(&self, u: &UserIdentity) -> Result<Token> {
        return self.token_repository.create(&Token::new(u.id)?).await
    }
}
