use std::collections::HashMap;
use std::path::PathBuf;
use super::traits::FileHandler;
use async_trait::async_trait;
use crate::util::error::Result;

#[derive(Clone)]
pub struct RpmFileHandler {

}


impl RpmFileHandler {
    pub fn new() -> Self {
        Self {

        }
    }
}

#[async_trait]
impl FileHandler for RpmFileHandler {
    fn get_sign_options(&self) -> HashMap<String, String> {
        HashMap::new()
    }

    async fn split_data(&self, path: &PathBuf) -> Result<Vec<Vec<u8>>> {
        todo!()
    }

    async fn assemble_data(&self, path: &PathBuf, data: Vec<Vec<u8>>, temp_dir: &PathBuf) -> Result<(String, String)> {
        todo!()
    }
}

