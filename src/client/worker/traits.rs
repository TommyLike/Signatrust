use async_trait::async_trait;
use crate::client::sign_identity::SignIdentity;
use async_channel::{Sender, SendError};
use crate::client::file_handler::factory::FileHandlerFactory;
use crate::client::file_handler::traits::FileHandler;

#[async_trait]
pub trait SignHandler {
    async fn handle(&mut self, item: SignIdentity, sender: Sender<SignIdentity>) -> () {
        if item.error.borrow().clone().is_err() {
            match sender.send(item).await {
                Err(err) => {
                    error!("failed to send sign object into channel: {}", err);
                }
                _ => {}
            };
        } else {
            let handler = FileHandlerFactory::get_handler(item.file_type.clone());
            let updated = self.process(handler, item).await;
            match sender.send(updated).await {
                Err(err) => {
                    error!("failed to send sign object into channel: {}", err);
                }
                _ => {}
            };
        }
    }
    //NOTE: instead of raise out error for specific sign object out of method, we need record error inside of the SignIdentity object.
    async fn process(&mut self, handler: Box<dyn FileHandler>, item: SignIdentity) -> SignIdentity;
}