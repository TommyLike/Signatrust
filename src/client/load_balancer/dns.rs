use tonic::transport::{Channel, Certificate, ClientTlsConfig, Identity};
use super::traits::DynamicLoadBalancer;
use tokio::time::{sleep, Duration};
use crate::util::error::Result;
use tonic::transport::Endpoint;
use async_trait::async_trait;
use tokio::fs;
use std::net::IpAddr;
use dns_lookup::{lookup_host, lookup_addr};

pub struct DNSLoadBalancer {
    hostname: String,
    port: String,
    client_config: Option<ClientTlsConfig>
}

impl DNSLoadBalancer {

    pub fn new(hostname: String, port: String, client_config: Option<ClientTlsConfig>) -> Result<Self> {
        Ok(Self {
            hostname,
            port,
            client_config
        })
    }

}

#[async_trait]
impl DynamicLoadBalancer for DNSLoadBalancer {
    fn get_transport_channel(&self) -> Result<Channel> {
        let mut endpoints = Vec::new();
        for ip in lookup_host(&self.hostname)?.into_iter() {
            let mut endpoint = Endpoint::from_shared(
                format!("http://{}:{}", ip, self.port))?;
            if let Some(tls_config) = self.client_config.clone() {
                endpoint = endpoint.tls_config(tls_config)?;
            }
            info!("found endpoint {}:{} for signing task.", ip, self.port);
            endpoints.push(endpoint);
        }
        Ok(Channel::balance_list(endpoints.into_iter()))
    }
}