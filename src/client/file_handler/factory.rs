use super::rpm::RpmFileHandler;
use super::checksum::CheckSumFileHandler;
use crate::client::sign_identity::FileType;
use super::traits::FileHandler;

pub struct FileHandlerFactory {
}

impl FileHandlerFactory {
    pub fn get_handler(file_type: FileType) -> Box<dyn FileHandler> {
        match file_type {
            FileType::RPM => {
                Box::new(RpmFileHandler::new())
            },
            FileType::CheckSum => {
                Box::new(CheckSumFileHandler::new())
            }
        }
    }
}