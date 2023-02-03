use std::collections::HashMap;
use async_trait::async_trait;
use std::path::PathBuf;
use crate::util::error::Result;

#[async_trait]
pub trait FileHandler: Send + Sync {
    fn get_sign_options(&self) -> HashMap<String, String>;
    async fn split_data(&self, path: &PathBuf) -> Result<Vec<Vec<u8>>>;
    //return the temporary file path and signature file name
    async fn assemble_data(&self, path: &PathBuf, data: Vec<Vec<u8>>, temp_dir: &PathBuf) -> Result<(String, String)>;
}