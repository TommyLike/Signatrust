use std::collections::HashMap;

use md5::{Digest};
use std::sync::Arc;
pub mod signatrust {
    tonic::include_proto!("signatrust");
}
use tokio_stream::StreamExt;

use signatrust::{
    signatrust_server::Signatrust, signatrust_server::SignatrustServer, SignStreamRequest,
    SignStreamResponse,
};
use tonic::{Request, Response, Status, Streaming};
use crate::infra::database::model::datakey::repository::EncryptedDataKeyRepository;


use crate::util::error::Result as InnerResult;
use crate::util::signer_container::SignerContainer;

pub struct SignService {
    data_key_repository: Arc<EncryptedDataKeyRepository>,
    container: SignerContainer
}

impl SignService {
    pub fn new(data_key_repository: Arc<EncryptedDataKeyRepository>) -> SignService {
        SignService {
            data_key_repository: data_key_repository.clone(),
            container: SignerContainer::new(data_key_repository),
        }
    }
    async fn sign_stream_inner(&self, key_type: String, key_name: String, options: &HashMap<String, String>, data: Vec<u8>) -> InnerResult<Vec<u8>> {
        self.container.get_signer(key_type, key_name).await?.sign(data.clone(), options.clone())
    }
}

#[tonic::async_trait]
impl Signatrust for SignService {
    async fn sign_stream(
        &self,
        request: Request<Streaming<SignStreamRequest>>,
    ) -> Result<Response<SignStreamResponse>, Status> {
        let mut binaries = request.into_inner();
        let mut data: Vec<u8> = vec![];
        let mut key_name: String = "".to_string();
        let mut key_type: String = "".to_string();
        let mut options: HashMap<String, String> = HashMap::new();
        while let Some(content) = binaries.next().await {
            let mut inner_result = content.unwrap();
            data.append(&mut inner_result.data);
            key_name = inner_result.key_id;
            key_type = inner_result.key_type;
            options = inner_result.options;
        }
        match self.sign_stream_inner(key_type, key_name, &options, data).await {
            Ok(content) => {
                Ok(Response::new(SignStreamResponse {
                    signature: content,
                    error: "".to_string()
                }))
            }
            Err(err) => {
                Ok(Response::new(SignStreamResponse {
                    signature: vec![],
                    error: err.to_string(),
                }))
            }
        }
    }
}

pub fn get_grpc_service(data_key_repository: Arc<EncryptedDataKeyRepository>) -> SignatrustServer<SignService> {
    let app = SignService::new(data_key_repository);
    SignatrustServer::new(app)
}
