use md5::{Digest, Md5};
pub mod signatrust {
    tonic::include_proto!("signatrust");
}
use tokio_stream::StreamExt;

use signatrust::{
    signatrust_server::Signatrust, signatrust_server::SignatrustServer, SignStreamRequest,
    SignStreamResponse,
};
use tonic::{Request, Response, Status, Streaming};

pub struct SignService {}

impl SignService {
    pub fn new() -> SignService {
        SignService {}
    }
}

#[tonic::async_trait]
impl Signatrust for SignService {
    async fn sign_stream(
        &self,
        request: Request<Streaming<SignStreamRequest>>,
    ) -> Result<Response<SignStreamResponse>, Status> {
        let mut binaries = request.into_inner();
        let mut hasher = Md5::new();
        while let Some(content) = binaries.next().await {
            hasher.update(&content.unwrap().data)
        }
        let hash_result = hasher.finalize();
        Ok(Response::new(SignStreamResponse {
            signature: format!("{:x}", hash_result),
            error_code: 0,
        }))
    }
}

pub fn get_grpc_service() -> SignatrustServer<SignService> {
    let app = SignService::new();
    SignatrustServer::new(app)
}
