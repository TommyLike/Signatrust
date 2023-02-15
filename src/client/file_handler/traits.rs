use std::collections::HashMap;
use async_trait::async_trait;
use std::path::PathBuf;
use crate::util::error::Result;
use tokio::fs;

#[async_trait]
pub trait FileHandler: Send + Sync {

    fn validate_options(&self, sign_options: &HashMap<String, String>) -> Result<()> {
        Ok(())
    }
    async fn split_data(&self, path: &PathBuf, sign_options: &mut HashMap<String, String>) -> Result<Vec<Vec<u8>>> {
        let content = fs::read(path).await?;
        Ok(vec![content])
    }
    //return the temporary file path and signature file name
    async fn assemble_data(&self, path: &PathBuf, data: Vec<Vec<u8>>, temp_dir: &PathBuf, sign_options: &HashMap<String, String>) -> Result<(String, String)>;
}