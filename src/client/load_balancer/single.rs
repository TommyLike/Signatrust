use tonic::transport::{Channel, Certificate, ClientTlsConfig, Identity};
use super::traits::DynamicLoadBalancer;
use tokio::time::{sleep, Duration};
use crate::util::error::Result;
use tonic::transport::Endpoint;
use async_trait::async_trait;
use tokio::fs;

pub struct SingleLoadBalancer {
    server: String,
    port: String,
    client_config: Option<ClientTlsConfig>
}

impl SingleLoadBalancer {
    pub fn new(server: String, port: String, client_config: Option<ClientTlsConfig>) -> Result<Self> {
        Ok(Self {
            server,
            port,
            client_config
        })
    }

}

#[async_trait]
impl DynamicLoadBalancer for SingleLoadBalancer {
    fn get_transport_channel(&self) -> Result<Channel> {
        let mut endpoint = Endpoint::from_shared(
            format!("http://{}:{}", self.server, self.port))?;
        if let Some(tls_config) = self.client_config.clone() {
            endpoint = endpoint.tls_config(tls_config)?
        }
        Ok(Channel::balance_list(vec![endpoint].into_iter()))
    }
}