use std::path::PathBuf;
use std::fmt::{Display, Formatter, Result as fmtResult, write};
use config::File;
use std::cell::{Cell, RefCell};
use crate::util::error::Result;
use std::collections::HashMap;

#[derive(clap::ValueEnum, Clone, Debug, PartialEq, Eq, Hash)]
pub enum FileType {
    RPM,
    CheckSum,
}

impl Display for FileType {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        match self {
            FileType::RPM => write!(f, "rpm"),
            FileType::CheckSum => write!(f, "checksum"),
        }
    }
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum KeyType {
    PGP,
    X509,
}

impl Display for KeyType {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        match self {
            KeyType::PGP => write!(f, "pgp"),
            KeyType::X509 => write!(f, "x509"),
        }
    }
}

pub struct SignIdentity {
    pub file_path: PathBuf,
    pub key_type: KeyType,
    pub file_type: FileType,
    pub key_id: String,
    pub raw_content: Box<RefCell<Vec<Vec<u8>>>>,
    pub signature: Box<RefCell<Vec<Vec<u8>>>>,
    pub sign_options: RefCell<HashMap<String, String>>,
    pub error: RefCell<Result<()>>,
}

impl SignIdentity {
    pub(crate) fn new(file_type: FileType, file_path: PathBuf, key_type: KeyType, key_id: String) -> Self {
        return Self {
            file_type,
            file_path,
            key_type,
            key_id,
            raw_content: Box::new(RefCell::new(vec![])),
            signature: Box::new(RefCell::new(vec![])),
            sign_options: RefCell::new(HashMap::new()),
            error: RefCell::new(Ok(())),
        }
    }
}
