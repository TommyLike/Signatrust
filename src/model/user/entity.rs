use crate::util::error::Result;


use std::fmt::{Display, Formatter};



#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub email: String

}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}, email: {}",
            self.id, self.email
        )
    }
}

impl User {
    pub fn new(email: String) -> Result<Self> {
        Ok(User {
            id: 0,
            email,
        })
    }
}
