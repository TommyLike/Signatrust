use actix_web::{
    HttpResponse, Responder, Result, web, Scope
};
use secstr::SecVec;
use serde::Serialize;
use crate::infra::database::model::datakey::repository::EncryptedDataKeyRepository;
use crate::service::control_service::model::datakey::dto::{DataKeyDTO, ExportKey};
use crate::util::error::Error;
use validator::Validate;
use crate::infra::sign::signers::Signers;
use crate::model::datakey::entity::DataKey;
use crate::model::datakey::repository::Repository;

async fn create_data_key(repository: web::Data<EncryptedDataKeyRepository>, datakey: web::Json<DataKeyDTO>,) -> Result<impl Responder, Error> {
    datakey.validate()?;
    let mut key = DataKey::try_from(datakey.0)?;
    let (private_key, public_key, certificate) =
        Signers::generate_keys(&key.key_type, &key.attributes)?;
    key.private_key = SecVec::new(private_key);
    key.public_key = SecVec::new(public_key);
    key.certificate = SecVec::new(certificate);
    Ok(HttpResponse::Created().json(DataKeyDTO::try_from(repository.into_inner().create(&key).await?)?))
}

async fn list_data_key(repository: web::Data<EncryptedDataKeyRepository>) -> Result<impl Responder, Error> {
    let keys = repository.into_inner().get_all().await?;
    let mut results = vec![];
    for k in keys {
        results.push(DataKeyDTO::try_from(k)?)
    }
    Ok(HttpResponse::Ok().json(results))
}

async fn show_data_key(repository: web::Data<EncryptedDataKeyRepository>, id: web::Path<String>) -> Result<impl Responder, Error> {
    let key = repository.into_inner().get_by_id(id.parse::<i32>()?).await?;
    Ok(HttpResponse::Ok().json(DataKeyDTO::try_from(key)?))
}

async fn delete_data_key(repository: web::Data<EncryptedDataKeyRepository>, id: web::Path<String>) -> Result<impl Responder, Error> {
    let key = repository.into_inner().delete_by_id(id.parse::<i32>()?).await?;
    Ok(HttpResponse::Ok())
}

async fn export_data_key(repository: web::Data<EncryptedDataKeyRepository>, id: web::Path<String>) -> Result<impl Responder, Error> {
    let key = repository.into_inner().get_by_id(id.parse::<i32>()?).await?;
    let exported = ExportKey::try_from(key)?;
    println!("{}", exported.public_key);
    println!("{}", exported.private_key);
    Ok(HttpResponse::Ok().json(exported))
}

async fn import_data_key(repository: web::Data<EncryptedDataKeyRepository>) -> Result<impl Responder, Error> {
    Ok(HttpResponse::Ok())
}


pub fn get_scope() -> Scope {
    web::scope("/datakeys")
        .service(
            web::resource("/")
                .route(web::get().to(list_data_key))
                .route(web::post().to(create_data_key)))
        .service( web::resource("/{id}")
            .route(web::get().to(show_data_key))
            .route(web::delete().to(delete_data_key)))
        .service( web::resource("/import").route(web::post().to(import_data_key)))
        .service( web::resource("/{id}/export").route(web::post().to(export_data_key)))
}
