use actix_web::{
   HttpResponse, Responder, Result, web, Scope
};
use serde::Serialize;
use crate::infra::database::model::datakey::repository::EncryptedDataKeyRepository;
use crate::util::error::Error;

#[derive(Serialize)]
struct FakeObject {
    id: String,
}

async fn get_user(repository: web::Data<EncryptedDataKeyRepository>, id: web::Path<String>) -> Result<impl Responder, Error> {
    let obj = FakeObject {
        id: id.to_string(),
    };
    Ok(web::Json(obj))
}

pub fn get_scope() -> Scope {
    web::scope("/users")
        .service(
            web::resource("/{id}").route(web::get().to(get_user)))
}