use crate::infra::sign_plugin::openpgp::OpenPGPPlugin;
use crate::infra::sign_plugin::x509::X509Plugin;
use crate::infra::sign_plugin::traits::SignPlugins;
use crate::model::datakey::entity::{KeyType};
use crate::util::error::Result;
use std::collections::HashMap;
use std::sync::Arc;
use crate::infra::sign_backend::sec_key::SecKey;

pub struct Signers {}

impl Signers {

    //get responding sign plugin for data signing
    pub fn load_from_data_key(key_type: &KeyType, data_key: &SecKey) -> Result<Arc<Box<dyn SignPlugins>>> {
        match key_type {
            KeyType::OpenPGP => Ok(Arc::new(Box::new(OpenPGPPlugin::new(data_key)?))),
            KeyType::X509 => Ok(Arc::new(Box::new(X509Plugin::new(data_key)?))),
        }
    }

    //generating new key, including private & public keys and the certificate, empty if not required.
    pub fn generate_keys(
        key_type: &KeyType,
        value: &HashMap<String, String>,
    ) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
        match key_type {
            KeyType::OpenPGP => OpenPGPPlugin::generate_keys(value),
            KeyType::X509 => X509Plugin::generate_keys(value),
        }
    }
}
