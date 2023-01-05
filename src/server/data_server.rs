use std::sync::{Arc, RwLock, atomic::AtomicBool};
use config::Config;
use std::fs;
use std::net::SocketAddr;
use std::sync::atomic::Ordering;
use tonic::{
    transport::{
        server::{TcpConnectInfo, TlsConnectInfo},
        Identity, Server, ServerTlsConfig,
    },
    Request, Response, Status,
};

use crate::service::grpc_service::get_grpc_service;
use crate::infra::kms::factory;
use crate::infra::database::pool::{create_pool, get_db_pool};
use crate::util::error::Result;
use crate::infra::database::model::clusterkey::repository;
use crate::model::clusterkey::entity::ClusterKey;
use crate::model::clusterkey::repository::Repository;

pub struct DataServer {
    server_config: Arc<RwLock<Config>>,
    signal: Arc<AtomicBool>,
    server_identity: Option<Identity>
}

impl DataServer {
    pub fn new(server_config: Arc<RwLock<Config>>, signal: Arc<AtomicBool>) -> Result<Self> {
        let mut server = DataServer{
            server_config,
            signal,
            server_identity:None,
        };
        server.load()?;
        Ok(server)
    }

    fn load(&mut self) -> Result<()> {
        if self.server_config.read()?.get_string(
            "server_tls_cert")?.is_empty() ||
            self.server_config.read()?.get_string(
                "server_tls_key")?.is_empty() {
            info!("tls key and cert not configured, data server tls will be disabled");
            return Ok(());
        }
        let key = fs::read(
            self.server_config.read()?.get_string("server_tls_key")?)?;
        let cert = fs::read(
            self.server_config.read()?.get_string("server_tls_cert")?)?;
        self.server_identity = Some(Identity::from_pem(cert, key));
        Ok(())
    }

    async fn shutdown_signal(&self) {
        while !self.signal.load(Ordering::Relaxed) {
        }
        info!("quit signal received...")
    }

    pub async fn run(&self) -> Result<()>{
        //initialize database and kms backend
        let kms_provider = factory::KMSProviderFactory::new_provider(
            &self.server_config.read()?.get_table("kms-provider")?)?;
        let encoded = kms_provider.encode("husheng".to_string()).await?;
        info!("encoded string {}", encoded);
        let decoded = kms_provider.decode(encoded).await?;
        info!("decoded string {}", decoded);
        let database = self.server_config.read()?.get_table("database")?;
        create_pool(&database).await?;
        //let repository  = repository::ClusterKeyRepository::new(get_db_pool()?);
        // let cluster_key = ClusterKey::default();
        // repository.create(&cluster_key).await?;
        // let cluster_a = repository.get_latest().await?;
        // println!("the latest cluster key is {:?}", cluster_a);
        // let cluster_b = repository.get_by_id(cluster_a.id).await?;
        // // println!("the 1st cluster key is {:?}", cluster_b.id);
        // // repository.delete_by_id(1).await?;

        //initialize signature plugins


        //start grpc server
        let addr: SocketAddr = format!(
            "{}:{}", self.server_config.read()?.get_string("data-server.server_ip")?,
            self.server_config.read()?.get_string(
                "data-server.server_port")?).parse()?;

        let mut server = Server::builder();
        info!("data server starts");
        if let Some(identity) = self.server_identity.clone() {
            server.tls_config(
                ServerTlsConfig::new().identity(identity))?
                .add_service(get_grpc_service()).serve_with_shutdown(addr, self.shutdown_signal()).await?
        } else {
            server
                .add_service(get_grpc_service()).serve_with_shutdown(addr, self.shutdown_signal()).await?
        }
        Ok(())
    }
}



