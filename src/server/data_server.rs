use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, atomic::AtomicBool, RwLock};
use std::sync::atomic::Ordering;

use config::Config;
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
use crate::service::data_service::grpc_service::get_grpc_service;
use crate::util::error::Error::KeyParseError;
use crate::util::error::Result;

pub struct DataServer {
    server_config: Arc<RwLock<Config>>,
    signal: Arc<AtomicBool>,
    server_identity: Option<Identity>,
    ca_cert: Option<Certificate>,
    data_key_repository: Arc<datakeyRepository::EncryptedDataKeyRepository>
}

impl DataServer {
    pub async fn new(server_config: Arc<RwLock<Config>>, signal: Arc<AtomicBool>) -> Result<Self> {
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
        let mut server = DataServer {
            server_config,
            signal,
            server_identity: None,
            ca_cert: None,
            data_key_repository: Arc::new(data_repository)
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
        // //initialize database and kms backend
        // let kms_provider = factory::KMSProviderFactory::new_provider(
        //     &self.server_config.read()?.get_table("kms-provider")?,
        // )?;
        // let encoded = kms_provider.encode("husheng".to_string()).await?;
        // info!("encoded string {}", encoded);
        // let decoded = kms_provider.decode(encoded).await?;
        // info!("decoded string {}", decoded);
        // let database = self.server_config.read()?.get_table("database")?;
        // create_pool(&database).await?;
        // let repository =
        //     repository::EncryptedClusterKeyRepository::new(get_db_pool()?, kms_provider.clone());
        // //initialize signature plugins
        // let engine_config = self.server_config.read()?.get_table("encryption-engine")?;
        // let mut engine = EncryptionEngineWithClusterKey::new(
        //     Arc::new(Box::new(repository.clone())),
        //     &engine_config,
        // )?;
        // engine.initialize().await?;
        // let value = vec![1, 2, 3, 4];
        // let result = engine.encode(value).await?;
        // let value2 = engine.decode(result).await?;
        // let data_repository = datakeyRepository::EncryptedDataKeyRepository::new(
        //     get_db_pool()?,
        //     Arc::new(Box::new(engine)),
        // );
        let mut parmaters: HashMap<String, String> = HashMap::new();
        parmaters.insert("name".to_string(), "openEuler".to_string());
        parmaters.insert("email".to_string(), "contact@openeuler.org".to_string());
        parmaters.insert("key_type".to_string(), "rsa".to_string());
        parmaters.insert("key_length".to_string(), "2048".to_string());
        let (private_key, public_key, certificate) =
            Signers::generate_keys(KeyType::OpenPGP, parmaters)?;
        println!("starting to save data keys");
        let mut data_key = DataKey {
            id: 0,
            name: "openEuler2003".to_string(),
            description: "openEuler".to_string(),
            user: "121".to_string(),
            email: "12121".to_string(),
            attributes: HashMap::new(),
            key_type: KeyType::OpenPGP,
            private_key: SecVec::new(vec![]),
            public_key:  SecVec::new(vec![]),
            certificate:  SecVec::new(vec![]),
            create_at: Default::default(),
            expire_at: Default::default(),
        };
        if let Some(k) = private_key {
            data_key.private_key = SecVec::new(k);
        }
        if let Some(k) = public_key {
            data_key.public_key = SecVec::new(k);
        }
        self.data_key_repository.create(&data_key).await?;
        let data_key = self.data_key_repository.get_by_id(1).await?;
        println!("{}", String::from_utf8_lossy(&data_key.public_key.unsecure()));
        // let content = vec![1, 2, 3, 4];
        // let result = signer.sign(content);

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
                .add_service(get_grpc_service(self.data_key_repository.clone()))
                .serve_with_shutdown(addr, self.shutdown_signal())
                .await?
        } else {
            server
                .add_service(get_grpc_service(self.data_key_repository.clone()))
                .serve_with_shutdown(addr, self.shutdown_signal())
                .await?
        }
        Ok(())
    }
}
