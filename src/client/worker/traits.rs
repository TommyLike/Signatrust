use async_trait::async_trait;
use crate::client::sign_identity::SignIdentity;
use async_channel::{Sender, SendError};
use crate::client::file_handler::factory::FileHandlerFactory;
use crate::client::file_handler::traits::FileHandler;

#[async_trait]
pub trait SignHandler {
    async fn handle(&mut self, item: SignIdentity, sender: Sender<SignIdentity>) -> () {
        if item.error.borrow().clone().is_err() {
            if let Err(err) = sender.send(item).await {
                error!("failed to send sign object into channel: {}", err);
            }
        } else {
            let handler = FileHandlerFactory::get_handler(item.file_type.clone());
            let updated = self.process(handler, item).await;
            if let Err(err) = sender.send(updated).await {
                error!("failed to send sign object into channel: {}", err);
            }
        }
    }
    //NOTE: instead of raise out error for specific sign object out of method, we need record error inside of the SignIdentity object.
    async fn process(&mut self, handler: Box<dyn FileHandler>, item: SignIdentity) -> SignIdentity;
}