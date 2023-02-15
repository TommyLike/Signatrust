use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, atomic::AtomicBool, RwLock};
use std::sync::atomic::Ordering;

use actix_web::{App, HttpServer, middleware, web};
use config::Config;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use secstr::*;
use tokio::fs;
use tokio::time::{Duration, sleep};
use tonic::{
    Request,
    Response, Status, transport::{
        Certificate,
        Identity, server::{TcpConnectInfo, TlsConnectInfo}, Server, ServerTlsConfig,
    },
};

use crate::infra::cipher::algorithm::{aes::Aes256GcmEncryptor, traits::Encryptor};
use crate::infra::cipher::engine::{EncryptionEngine, EncryptionEngineWithClusterKey};
use crate::infra::database::model::clusterkey::repository;
use crate::infra::database::model::datakey::repository as datakeyRepository;
use crate::infra::database::pool::{create_pool, get_db_pool};
use crate::infra::kms::factory;
use crate::infra::sign::signers::Signers;
use crate::model::clusterkey::entity::ClusterKey;
use crate::model::clusterkey::repository::Repository;
use crate::model::datakey::entity::{DataKey, KeyType};
use crate::model::datakey::repository::Repository as DatakeyRepository;
use crate::service::control_service::*;
use crate::util::error::Error::KeyParseError;
use crate::util::error::Result;

pub struct ControlServer {
    server_config: Arc<RwLock<Config>>,
    data_key_repository: web::Data<datakeyRepository::EncryptedDataKeyRepository>,
}

impl ControlServer {
    pub async fn new(server_config: Arc<RwLock<Config>>) -> Result<Self> {
        //initialize database and kms backend
        let kms_provider = factory::KMSProviderFactory::new_provider(
            &server_config.read()?.get_table("kms-provider")?,
        )?;
        let database = server_config.read()?.get_table("database")?;
        create_pool(&database).await?;
        let repository =
            repository::EncryptedClusterKeyRepository::new(get_db_pool()?, kms_provider.clone());
        //initialize signature plugins
        let engine_config = server_config.read()?.get_table("encryption-engine")?;
        let mut engine = EncryptionEngineWithClusterKey::new(
            Arc::new(Box::new(repository.clone())),
            &engine_config,
        )?;
        engine.initialize().await?;
        let data_repository = datakeyRepository::EncryptedDataKeyRepository::new(
            get_db_pool()?,
            Arc::new(Box::new(engine)),
        );
        let server = ControlServer {
            server_config,
            data_key_repository: web::Data::new(data_repository),
        };
        Ok(server)
    }

    pub async fn run(&self) -> Result<()> {
        //start actix web server
        let addr: SocketAddr = format!(
            "{}:{}",
            self.server_config
                .read()?
                .get_string("control-server.server_ip")?,
            self.server_config
                .read()?
                .get_string("control-server.server_port")?
        )
            .parse()?;

        info!("control server starts");
        // Start http server
        let data_key_repository = self.data_key_repository.clone();
        let http_server = HttpServer::new(move || {
            App::new()
                // enable logger
                .app_data(data_key_repository.clone())
                .wrap(middleware::Logger::default())
                .service(web::scope("/api/v1")
                    .service(user_service::get_scope())
                    .service(datakey_service::get_scope()))
        });
        if self.server_config
            .read()?
            .get_string("tls_cert")?
            .is_empty()
            || self
            .server_config
            .read()?
            .get_string("tls_key")?
            .is_empty() {
            info!("tls key and cert not configured, control server tls will be disabled");
            http_server.bind(addr)?.run().await?;
        } else {
            let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
            builder
                .set_private_key_file(
                    self.server_config.read()?.get_string("tls_key")?, SslFiletype::PEM).unwrap();
            builder.set_certificate_chain_file(
                self.server_config.read()?.get_string("tls_cert")?).unwrap();
            http_server.bind_openssl(addr, builder)?.run().await?;
        }
        Ok(())
    }
}
