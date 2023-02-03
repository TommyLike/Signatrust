use std::ffi::OsStr;
use crate::client::{file_handler, sign_identity::SignIdentity};
use crate::util::error::Result;
use async_channel::Sender;
use crate::client::worker::traits::SignHandler;
use crate::client::file_handler::traits::FileHandler;
use async_trait::async_trait;
use std::path::{Path, PathBuf};
use std::fs::rename;
use crate::util::error::Error;
use std::cell::RefCell;
use std::fs;
use clap::builder::Str;

pub struct Assembler {
    temp_dir: PathBuf
}


impl Assembler {

    pub fn new(temp_dir: String) -> Self {
        Self {
            temp_dir: PathBuf::from(temp_dir)
        }
    }

}

#[async_trait]
impl SignHandler for Assembler {
    //file handler used to generate signed file in temp folder and assembler will move the signed file back
    async fn process(&mut self, handler: Box<dyn FileHandler>, item: SignIdentity) -> SignIdentity {
        let signatures: Vec<Vec<u8>> = (*item.signature).borrow().clone();
        match handler.assemble_data(&item.file_path,  signatures, &self.temp_dir).await {
            Ok(content) => {
                debug!("successfully assemble file {}", item.file_path.as_path().display());
                let temp_file = Path::new(&content.0);
                let base_directory =item.file_path.as_path().parent();
                match base_directory {
                    None => {
                        *item.error.borrow_mut() = Err(Error::AssembleFileError(format!("failed to get base directory of signed file {}", item.file_path.display())))
                    }
                    Some(directory) => {
                        let signed_file = directory.join(content.1);
                        match rename(temp_file, signed_file) {
                            Ok(_) => {
                                debug!("successfully assemble file {}", item.file_path.as_path().display());
                            }
                            Err(err) => {
                                *item.error.borrow_mut() = Err(Error::AssembleFileError(format!("{:?}", err)))
                            }
                        }
                    }
                }
                //remove temp file when finished
                let _ = fs::remove_file(temp_file);
            }
            Err(err) => {
                *item.error.borrow_mut() = Err(Error::AssembleFileError(format!("{:?}", err)))
            }
        }
        item
    }
}