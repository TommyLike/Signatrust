use actix_web::{HttpResponse, Responder, Result, web, Scope, HttpRequest, HttpMessage};

use serde::{Deserialize, Serialize};

use crate::util::error::Error;
use super::model::user::dto::UserIdentity;
use actix_identity::Identity;
use openidconnect::{AdditionalClaims, UserInfoClaims};
use openidconnect::{
    AuthenticationFlow, AuthorizationCode, CsrfToken, Nonce,
    OAuth2TokenResponse, core::CoreResponseType, core::CoreClient, core::CoreGenderClaim
};

use openidconnect::Scope as OIDCScore;
use openidconnect::reqwest::async_http_client;


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
        .request_async(async_http_client).await {
        Ok(token_response) => {
            let userinfo_claim_result: UserInfoClaims<EmailClaims, CoreGenderClaim> = client
                .user_info(token_response.access_token().to_owned(), None)?
                .request_async(async_http_client).await?;
            Identity::login(&req.extensions(),
                            userinfo_claim_result.additional_claims().email.clone()).unwrap();
            Ok(HttpResponse::Found().insert_header(("Location", "/")).finish())
        }
        Err(err) => {
            Err(Error::AuthError(format!("failed to get oidc token {}", err)))
        }
    }
}



pub fn get_scope() -> Scope {
    web::scope("/users")
        .service(web::resource("/").route(web::get().to(info)))
        .service(web::resource("/login").route(web::get().to(login)))
        .service(web::resource("/logout").route(web::post().to(logout)))
        .service(web::resource("/callback").route(web::get().to(callback)))
}