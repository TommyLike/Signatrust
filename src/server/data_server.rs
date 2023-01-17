use config::Config;
use std::fs;
use std::net::SocketAddr;
use std::sync::atomic::Ordering;
use std::sync::{atomic::AtomicBool, Arc, RwLock};
use tonic::{
    transport::{
        server::{TcpConnectInfo, TlsConnectInfo},
        Identity, Server, ServerTlsConfig,
    },
    Request, Response, Status,
};

use crate::infra::cipher::algorithm::{aes::Aes256GcmEncryptor, traits::Encryptor};
use crate::infra::cipher::engine::{EncryptionEngine, EncryptionEngineWithClusterKey};
use crate::infra::database::model::clusterkey::repository;
use crate::infra::database::pool::{create_pool, get_db_pool};
use crate::infra::kms::factory;
use crate::model::clusterkey::entity::ClusterKey;
use crate::model::clusterkey::repository::Repository;
use crate::service::grpc_service::get_grpc_service;
use crate::util::error::Result;

pub struct DataServer {
    server_config: Arc<RwLock<Config>>,
    signal: Arc<AtomicBool>,
    server_identity: Option<Identity>,
}

impl DataServer {
    pub fn new(server_config: Arc<RwLock<Config>>, signal: Arc<AtomicBool>) -> Result<Self> {
        let mut server = DataServer {
            server_config,
            signal,
            server_identity: None,
        };
        server.load()?;
        Ok(server)
    }

    fn load(&mut self) -> Result<()> {
        if self
            .server_config
            .read()?
            .get_string("server_tls_cert")?
            .is_empty()
            || self
                .server_config
                .read()?
                .get_string("server_tls_key")?
                .is_empty()
        {
            info!("tls key and cert not configured, data server tls will be disabled");
            return Ok(());
        }
        let key = fs::read(self.server_config.read()?.get_string("server_tls_key")?)?;
        let cert = fs::read(self.server_config.read()?.get_string("server_tls_cert")?)?;
        self.server_identity = Some(Identity::from_pem(cert, key));
        Ok(())
    }

    async fn shutdown_signal(&self) {
        while !self.signal.load(Ordering::Relaxed) {}
        info!("quit signal received...")
    }

    pub async fn run(&self) -> Result<()> {
        //initialize database and kms backend
        let kms_provider = factory::KMSProviderFactory::new_provider(
            &self.server_config.read()?.get_table("kms-provider")?,
        )?;
        let encoded = kms_provider.encode("husheng".to_string()).await?;
        info!("encoded string {}", encoded);
        let decoded = kms_provider.decode(encoded).await?;
        info!("decoded string {}", decoded);
        let database = self.server_config.read()?.get_table("database")?;
        create_pool(&database).await?;
        let repository =
            repository::EncryptedClusterKeyRepository::new(get_db_pool()?, kms_provider.clone());
        //initialize signature plugins
        let engine_config = self.server_config.read()?.get_table("encryption-engine")?;
        let mut engine = EncryptionEngineWithClusterKey::new(
            Arc::new(Box::new(repository.clone())),
            &engine_config,
        )?;
        engine.initialize().await?;
        let value = vec![1, 2, 3, 4];
        let result = engine.encode(value).await?;
        let value2 = engine.decode(result).await?;
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
                .tls_config(ServerTlsConfig::new().identity(identity))?
                .add_service(get_grpc_service())
                .serve_with_shutdown(addr, self.shutdown_signal())
                .await?
        } else {
            server
                .add_service(get_grpc_service())
                .serve_with_shutdown(addr, self.shutdown_signal())
                .await?
        }
        Ok(())
    }
}
