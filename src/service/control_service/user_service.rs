use actix_web::{HttpResponse, Responder, Result, web, Scope, HttpRequest, HttpMessage, FromRequest, dev::Payload};
use std::future::{ready, Ready};
use serde::{Deserialize, Serialize};
use crate::infra::database::model::datakey::repository::EncryptedDataKeyRepository;
use crate::util::error::Error;
use super::model::user::dto::UserIdentity;
use actix_identity::Identity;
use openidconnect::{AdditionalClaims, EmptyAdditionalClaims, UserInfoClaims};
use openidconnect::{
    AuthenticationFlow, AuthorizationCode, ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce,
    OAuth2TokenResponse, RedirectUrl, AuthUrl, UserInfoUrl, TokenUrl, JsonWebKeySet, core::CoreResponseType, core::CoreClient, core::CoreGenderClaim
};
use openidconnect::core::CoreTokenResponse;
use openidconnect::Scope as OIDCScore;
use openidconnect::reqwest::http_client;
use crate::util::error;

#[derive(Debug, Deserialize, Serialize)]
pub struct EmailClaims {
    pub email: String
}

impl AdditionalClaims for EmailClaims {}

#[derive(Deserialize)]
struct Code {
    pub code: String,
}

async fn login(client: web::Data<CoreClient>) -> Result<impl Responder, Error> {
    let (authorize_url, _, _) = client
        .authorize_url(AuthenticationFlow::<CoreResponseType>::AuthorizationCode,
                       CsrfToken::new_random, Nonce::new_random, )
        .add_scope(OIDCScore::new("email".to_string()))
        .add_scope(OIDCScore::new("openid".to_string()))
        .add_scope(OIDCScore::new("profile".to_string()))
        .url();
    Ok(HttpResponse::Found().insert_header(("Location", authorize_url.as_str())).finish())
}

async fn info(user: UserIdentity) -> Result<impl Responder, Error> {
    Ok(HttpResponse::Ok().json(user))
}

async fn logout(id: Identity) -> Result<impl Responder, Error> {
    id.logout();
    Ok( HttpResponse::NoContent().finish())
}

async fn callback(req: HttpRequest, client: web::Data<CoreClient>, code: web::Query<Code>) -> Result<impl Responder, Error> {
    match client
        .exchange_code(AuthorizationCode::new(code.code.clone()))
        .request(http_client) {
        Ok(token_response) => {
            let userinfo_claim_result: UserInfoClaims<EmailClaims, CoreGenderClaim> = client
                .user_info(token_response.access_token().to_owned(), None)?
                .request(http_client)?;
            Identity::login(&req.extensions(),
                            userinfo_claim_result.additional_claims().email.clone()).unwrap();
            Ok(HttpResponse::Found().insert_header(("Location", "/")).finish())
        }
        Err(err) => {
            return Err(Error::AuthError(format!("failed to get oidc token {}", err.to_string())))
        }
    }
}



pub fn get_scope() -> Scope {
    web::scope("/users")
        .service(web::resource("/").route(web::get().to(info)))
        .service(web::resource("/login").route(web::get().to(login)))
        .service(web::resource("/logout").route(web::post().to(logout)))
        .service(web::resource("/callback").route(web::post().to(callback)))
}