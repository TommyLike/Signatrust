use std::path::PathBuf;
use super::traits::FileHandler;
use async_trait::async_trait;
use crate::util::error::Result;
use tokio::fs;
use uuid::Uuid;
use std::io::Write;
use std::collections::HashMap;


const FILE_EXTENSION: &str = "asc";

#[derive(Clone)]
pub struct CheckSumFileHandler {

}

impl CheckSumFileHandler {
    pub fn new() -> Self {
        Self {

        }
    }
}

#[async_trait]
impl FileHandler for CheckSumFileHandler {

    fn get_sign_options(&self) -> HashMap<String, String> {
        HashMap::from([
            ("detached".to_string(), "true".to_string()),
        ])
    }
    //Read the whole content and await
    async fn split_data(&self, path: &PathBuf) -> Result<Vec<Vec<u8>>> {
        let content = fs::read(path).await?;
        Ok(vec![content])
    }

    /* when assemble checksum signature when only create another .asc file separately */
    async fn assemble_data(&self, path: &PathBuf, data: Vec<Vec<u8>>, temp_dir: &PathBuf) -> Result<(String, String)> {
        let temp_file = temp_dir.join(Uuid::new_v4().to_string());
        //convert bytes into string
        let result = String::from_utf8_lossy(&data[0]);
        fs::write(temp_file.clone(), result.as_bytes()).await?;
        Ok((temp_file.as_path().display().to_string(),
            format!("{}.{}", path.display(), FILE_EXTENSION)))
    }
}

