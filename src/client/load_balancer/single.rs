use tonic::transport::Channel;
use super::traits::DynamicLoadBalancer;
use tokio::time::{sleep, Duration};
use crate::util::error::Result;
use tonic::transport::Endpoint;
use async_trait::async_trait;

pub struct SingleLoadBalancer {
    server: String,
    port: String
}

impl SingleLoadBalancer {
    pub(crate) fn new(server: String, port: String) -> Self {
        Self {
            server,
            port
        }
    }

}

#[async_trait]
impl DynamicLoadBalancer for SingleLoadBalancer {
    fn get_transport_channel(&self) -> Result<Channel> {
        let endpoint = Endpoint::from_shared(
            format!("http://{}:{}", self.server, self.port))?;
        Ok(Channel::balance_list(vec![endpoint].into_iter()))
    }

    async fn refresh_endpoint(&self) -> Result<()> {
        loop {
            println!("hello world");
            sleep(Duration::from_secs(10)).await;
        }
    }
}