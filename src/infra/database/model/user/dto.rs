use crate::util::error::Result;



use sqlx::FromRow;





use crate::model::user::entity::User;

#[derive(Debug, FromRow)]
pub(super) struct UserDTO {
    pub id: i32,
    pub email: String
}

impl UserDTO {
    pub async fn encrypt(
        user: &User,
    ) -> Result<Self> {
        Ok(Self {
            id: user.id,
            email: user.email.clone(),
        })
    }
    pub async fn decrypt(&self) -> Result<User> {
        Ok(User {
            id: self.id,
            email: self.email.clone()
        })
    }
}
