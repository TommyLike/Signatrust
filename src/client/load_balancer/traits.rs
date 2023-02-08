use tonic::transport::Channel;
use crate::util::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait DynamicLoadBalancer {
    fn get_transport_channel(&self) -> Result<Channel>;
}