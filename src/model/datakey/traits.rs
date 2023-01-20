use crate::util::error::Result;

pub trait ExtendableAttributes {
    type Item;

    fn get_attributes(&self) -> Option<Self::Item>;
    fn serialize_attributes(&self) -> Result<String>;
}

pub trait Identity {
    fn get_identity(&self) -> String;
}
