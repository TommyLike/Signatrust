
use crate::client::{sign_identity::SignIdentity};


use crate::client::worker::traits::SignHandler;
use crate::client::file_handler::traits::FileHandler;
use async_trait::async_trait;
use std::path::{Path, PathBuf};
use std::fs::copy;
use crate::util::error::Error;


use std::fs;


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
        let sign_options = item.sign_options.borrow().clone();
        match handler.assemble_data(&item.file_path,  signatures, &self.temp_dir, &sign_options).await {
            Ok(content) => {
                debug!("successfully assemble file {}", item.file_path.as_path().display());
                let temp_file = Path::new(&content.0);
                match copy(temp_file, Path::new(&content.1)) {
                    Ok(_) => {
                        debug!("successfully saved file {}", item.file_path.as_path().display());
                    }
                    Err(err) => {
                        *item.error.borrow_mut() = Err(Error::AssembleFileError(format!("{:?}", err)));
                    }
                }
                //remove temp file when finished
                let _ = fs::remove_file(temp_file);
            }
            Err(err) => {
                *item.error.borrow_mut() = Err(Error::AssembleFileError(format!("{:?}", err)));
            }
        }
        item
    }
}