use crate::model::datakey::traits::{ExtendableAttributes, Identity};
use crate::util::error::{Error, Result};
use chrono::{DateTime, Duration, Utc};
use serde_json;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
#[derive(Debug)]
pub enum KeyType {
    OpenPGP,
}

impl FromStr for KeyType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "openpgp" => Ok(KeyType::OpenPGP),
            _ => Err(Error::UnsupportedTypeError(format!("{} data key type", s))),
        }
    }
}

impl Display for KeyType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            KeyType::OpenPGP => write!(f, "openpgp"),
        }
    }
}

#[derive(Debug)]
pub struct DataKey {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub user: String,
    pub email: String,
    pub attributes: HashMap<String, String>,
    pub key_type: KeyType,
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
    pub certificate: Vec<u8>,
    pub create_at: DateTime<Utc>,
    pub expire_at: DateTime<Utc>,
}

impl ExtendableAttributes for DataKey {
    type Item = HashMap<String, String>;

    fn get_attributes(&self) -> Option<Self::Item> {
        Some(self.attributes.clone())
    }

    fn serialize_attributes(&self) -> Result<String> {
        Ok(serde_json::to_string(&self.attributes)?)
    }
}

impl Identity for DataKey {
    fn get_identity(&self) -> String {
        format!(
            "<ID:{},Email:{},User:{},Type:{}>",
            self.id,
            self.email,
            self.user,
            self.key_type.to_string()
        )
    }
}
