use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use super::traits::FileHandler;
use async_trait::async_trait;
use crate::util::error::Result;
use std::fs::File;
use std::io::BufReader;
use rpm::{Header, IndexSignatureTag, RPMError, RPMPackage};
use crate::util::error;

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

    //rpm has two sections need to be signed
    //1. header
    //2. header and content
    async fn split_data(&self, path: &PathBuf) -> Result<Vec<Vec<u8>>> {
        todo!()
        //todo: support rpm file type when read&update rpm header outside of rpm crate
        // let data = Vec::new();
        // let file = File::open(path)?;
        // let mut package = RPMPackage::parse(&mut BufReader::new(file))?;
        // let content = package.content.as_slice();
        // let mut header_bytes = Vec::<u8>::with_capacity(1024);
        // package.metadata.header.write(&mut header_bytes)?;
        // package.metadata.signature = Header::<IndexSignatureTag>::new_signature_header(
        //     1 as i32,
        //     &digest_md5,
        //     digest_sha1,
        //     rsa_signature_spanning_header_only.as_slice(),
        //     rsa_signature_spanning_header_and_archive.as_slice(),
        // );
        //
        // Ok(vec![])
    }

    async fn assemble_data(&self, path: &PathBuf, data: Vec<Vec<u8>>, temp_dir: &PathBuf) -> Result<(String, String)> {
        todo!()
    }
}

