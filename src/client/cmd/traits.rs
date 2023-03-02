use crate::util::error::Result;
use std::sync::{RwLock, Arc, atomic::AtomicBool};
use config::Config;


pub trait SignCommand: Clone {
    type CommandValue;
    fn new(signal: Arc<AtomicBool>, config: Arc<RwLock<Config>>, command: Self::CommandValue) -> Result<Self>;
    fn validate(&self) -> Result<()>;
    fn handle(&self) -> Result<bool>;
}