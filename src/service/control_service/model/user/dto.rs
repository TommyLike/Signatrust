use actix_web::{Result, HttpRequest, FromRequest, dev::Payload};
use crate::util::error::{Error};
use std::future::{ready, Ready};
use actix_identity::Identity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserIdentity {
    pub email: String,
    pub username: String,
}

impl FromRequest for UserIdentity {
    type Error = Error;
    type Future = Ready<Result<UserIdentity, Error>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        if let Ok(identity) = Identity::from_request(req, pl).into_inner() {
            if let Ok(user_json) = identity.id() {
                if let Ok(user) = serde_json::from_str(&user_json) {
                    return ready(Ok(user));
                }
            }
        }
        ready(Err(Error::UnauthorizedError))
    }
}