use crate::infra::sign::openpgp::OpenPGPPlugin;
use crate::infra::sign::traits::SignPlugins;
use crate::model::datakey::entity::{DataKey, KeyType};
use crate::util::error::Result;
use std::collections::HashMap;
use std::sync::Arc;

pub struct Signers {}

impl Signers {
    //get responding sign plugin for data signing
    pub fn load_from_data_key(data_key: DataKey) -> Result<Arc<Box<dyn SignPlugins>>> {
        match data_key.key_type {
            KeyType::OpenPGP => Ok(Arc::new(Box::new(OpenPGPPlugin::new(data_key)?))),
        }
    }

    //generating new key
    pub fn generate_keys(
        key_type: KeyType,
        value: HashMap<String, String>,
    ) -> Result<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)> {
        match key_type {
            KeyType::OpenPGP => OpenPGPPlugin::generate_keys(value),
        }
    }
}