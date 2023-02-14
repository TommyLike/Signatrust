use crate::model::datakey::entity::DataKey;
use crate::util::error::Result;
use std::collections::HashMap;

pub trait SignPlugins: Send + Sync {
    fn new(db: DataKey) -> Result<Self>
    where
        Self: Sized;
    fn parse_attributes(
        private_key: Option<Vec<u8>>,
        public_key: Option<Vec<u8>>,
        certificate: Option<Vec<u8>>,
    ) -> HashMap<String, String>
    where
        Self: Sized;
    fn generate_keys(
        value: HashMap<String, String>,
    ) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)>
    where
        Self: Sized;
    fn sign(&self, content: Vec<u8>, options: HashMap<String, String>) -> Result<Vec<u8>>;
}
