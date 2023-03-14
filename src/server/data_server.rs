
use std::net::SocketAddr;
use std::sync::{Arc, atomic::AtomicBool, RwLock};
use std::sync::atomic::Ordering;

use config::Config;

use tokio::fs;
use tokio::time::{Duration, sleep};
use tonic::{
    transport::{
        Certificate,
        Identity, Server, ServerTlsConfig,
    },
};




use crate::infra::database::model::datakey::repository as datakeyRepository;
use crate::infra::database::pool::{create_pool, get_db_pool};

use crate::infra::sign_backend::factory::SignBackendFactory;
use crate::infra::sign_backend::traits::SignBackend;


use crate::service::data_service::grpc_service::get_grpc_service;

use crate::util::error::Result;

pub struct DataServer {
    server_config: Arc<RwLock<Config>>,
    signal: Arc<AtomicBool>,
    server_identity: Option<Identity>,
    ca_cert: Option<Certificate>,
    data_key_repository: Arc<datakeyRepository::DataKeyRepository>,
    sign_backend: Arc<Box<dyn SignBackend>>
}

impl DataServer {
    pub async fn new(server_config: Arc<RwLock<Config>>, signal: Arc<AtomicBool>) -> Result<Self> {
        let database = server_config.read()?.get_table("database")?;
        create_pool(&database).await?;
        let sign_backend = SignBackendFactory::new_engine(
            server_config.clone(), get_db_pool()?).await?;
        let data_repository = datakeyRepository::DataKeyRepository::new(
            get_db_pool()?);
        let mut server = DataServer {
            server_config,
            signal,
            server_identity: None,
            ca_cert: None,
            data_key_repository: Arc::new(data_repository),
            sign_backend: Arc::new(sign_backend)
        };
        server.load().await?;
        Ok(server)
    }

    async fn load(&mut self) -> Result<()> {
        if self
            .server_config
            .read()?
            .get_string("tls_cert")?
            .is_empty()
            || self
                .server_config
                .read()?
                .get_string("tls_key")?
                .is_empty()
        {
            info!("tls key and cert not configured, data server tls will be disabled");
            return Ok(());
        }
        self.ca_cert = Some(
            Certificate::from_pem(
                fs::read(self.server_config.read()?.get_string("ca_root")?).await?));
        self.server_identity = Some(Identity::from_pem(
            fs::read(self.server_config.read()?.get_string("tls_cert")?).await?,
            fs::read(self.server_config.read()?.get_string("tls_key")?).await?));
        Ok(())
    }

    async fn shutdown_signal(&self) {
        while !self.signal.load(Ordering::Relaxed) {
            sleep(Duration::from_secs(1)).await;
        }
        info!("quit signal received...")
    }

    pub async fn run(&self) -> Result<()> {
        //start grpc server
        let addr: SocketAddr = format!(
            "{}:{}",
            self.server_config
                .read()?
                .get_string("data-server.server_ip")?,
            self.server_config
                .read()?
                .get_string("data-server.server_port")?
        )
        .parse()?;

        let mut server = Server::builder();
        info!("data server starts");
        if let Some(identity) = self.server_identity.clone() {
            server
                .tls_config(ServerTlsConfig::new().identity(identity).client_ca_root(self.ca_cert.clone().unwrap()))?
                .add_service(get_grpc_service(self.data_key_repository.clone(), self.sign_backend.clone()))
                .serve_with_shutdown(addr, self.shutdown_signal())
                .await?
        } else {
            server
                .add_service(get_grpc_service(self.data_key_repository.clone(), self.sign_backend.clone()))
                .serve_with_shutdown(addr, self.shutdown_signal())
                .await?
        }
        Ok(())
    }
}
