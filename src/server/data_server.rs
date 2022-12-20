use std::error::Error;
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
        server.load().unwrap();
        Ok(server)
    }

    fn load(&mut self) -> Result<()> {
        if self.server_config.read().unwrap().get_string(
            "server_tls_cert").unwrap().is_empty() ||
            self.server_config.read().unwrap().get_string(
                "server_tls_key").unwrap().is_empty() {
            info!("tls key and cert not configured, data server tls will be disabled");
            return Ok(());
        }
        let key = fs::read(
            self.server_config.read().unwrap().get_string("server_tls_key").unwrap())?;
        let cert = fs::read(
            self.server_config.read().unwrap().get_string("server_tls_cert").unwrap())?;
        self.server_identity = Some(Identity::from_pem(cert, key));
        Ok(())
    }

    async fn shutdown_signal(&self) {
        while !self.signal.load(Ordering::Relaxed) {
        }
        info!("quit signal received...")
    }

    pub async fn run(&self) {
        //initialize database and kms backend
        let kms_provider = factory::KMSProviderFactory::new_provider(
            &self.server_config.read().unwrap().get_table("kms-provider").unwrap()).unwrap();
        create_pool(&self.server_config.read().unwrap().get_table("database").unwrap()).await.unwrap();
        //initialize signature plugins


        //start grpc server
        let addr: SocketAddr = format!(
            "{}:{}", self.server_config.read().unwrap().get_string("data-server.server_ip").unwrap(),
            self.server_config.read().unwrap().get_string(
                "data-server.server_port").unwrap()).parse().unwrap();

        let mut server = Server::builder();
        info!("data server starts");
        if let Some(identity) = self.server_identity.clone() {
            server.tls_config(
                ServerTlsConfig::new().identity(identity)).unwrap()
                .add_service(get_grpc_service()).serve_with_shutdown(addr, self.shutdown_signal()).await.unwrap()
        } else {
            server
                .add_service(get_grpc_service()).serve_with_shutdown(addr, self.shutdown_signal()).await.unwrap()
        }
    }
}



