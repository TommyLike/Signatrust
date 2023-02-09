use std::collections::HashMap;
use crate::client::{file_handler, sign_identity::SignIdentity};
use crate::util::error::Result;
use async_channel::Sender;
use crate::client::worker::traits::SignHandler;
use crate::client::file_handler::traits::FileHandler;
use async_trait::async_trait;
use crate::util::error;

pub struct Splitter {
}


impl Splitter {

    pub fn new() -> Self {
        Self {
        }
    }
}

#[async_trait]
impl SignHandler for Splitter {
    async fn process(&mut self, handler: Box<dyn FileHandler>, item: SignIdentity) -> SignIdentity {
        let mut sign_options = item.sign_options.borrow().clone();
        match handler.split_data(&item.file_path, &mut sign_options).await {
            Ok(content) => {
                *item.raw_content.borrow_mut() = content;
                *item.sign_options.borrow_mut() = sign_options;
                debug!("successfully split file {}", item.file_path.as_path().display());
            }
            Err(err) => {
                *item.error.borrow_mut() = Err(error::Error::SplitFileError(format!("{:?}", err)))
            }
        }
        item
    }
}

