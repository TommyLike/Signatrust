use actix_web::{HttpResponse, Responder, Result, web, Scope, HttpRequest, HttpMessage, FromRequest, dev::Payload};
use std::future::{ready, Ready};
use serde::{Deserialize, Serialize};
use crate::infra::database::model::datakey::repository::EncryptedDataKeyRepository;
use crate::util::error::Error;
use super::model::user::dto::UserIdentity;
use actix_identity::Identity;

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthData {
    pub email: String
}

async fn login(req: HttpRequest, repository: web::Data<EncryptedDataKeyRepository>, auth_data: web::Json<AuthData>) -> Result<impl Responder, Error> {
    Identity::login(&req.extensions(), auth_data.email.to_string()).unwrap();
    Ok(HttpResponse::NoContent().finish())
}

async fn logout(repository: web::Data<EncryptedDataKeyRepository>, id: Identity) -> Result<impl Responder, Error> {
    id.logout();
    Ok( HttpResponse::NoContent().finish())
}

async fn callback(repository: web::Data<EncryptedDataKeyRepository>, user: UserIdentity) -> Result<impl Responder, Error> {
    Ok( HttpResponse::NoContent().finish())
}



pub fn get_scope() -> Scope {
    web::scope("/users")
        .service(web::resource("/login").route(web::post().to(login)))
        .service(web::resource("/logout").route(web::post().to(logout)))
        .service(web::resource("/callback").route(web::post().to(callback)))
}