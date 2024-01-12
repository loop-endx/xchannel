pub mod dto;
pub mod table;
pub mod r#type;

use serde_derive::{Deserialize, Serialize};

use crate::error::XResult;

use r#type::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub value: Value,
    pub dtype: DataType,
    pub address: Option<String>,
    pub description: Option<String>,
}

impl Tag {
    pub fn new(tag: &dto::Tag) -> XResult<Tag> {
        let init_v = tag.dtype.init();

        if let Some(v) = &tag.value {
            if init_v.r#type() != v.r#type() {
                return Err(crate::error::XError::new(
                    crate::error::XErrorKind::TagError,
                    "Tag value type mismatch",
                ));
            }
        }

        Ok(Tag {
            name: tag.name.clone(),
            value: tag.value.clone().unwrap_or(init_v),
            dtype: tag.dtype.clone(),
            address: tag.address.clone(),
            description: tag.description.clone(),
        })
    }

    //pub fn update(&mut self, tag: &dto::Tag) -> XResult<()> {
    //let init_v = tag.dtype.init();

    //if let Some(v) = &tag.value {
    //if init_v.r#type() != v.r#type() {
    //return Err(crate::error::XError::new(
    //crate::error::XErrorKind::TagError,
    //"Tag value type mismatch",
    //));
    //}
    //}

    //self.value = tag.value.clone().unwrap_or(init_v);
    //self.dtype = tag.dtype.clone();
    //self.address = tag.address.clone();
    //self.description = tag.description.clone();

    //Ok(())
    //}

    //pub fn update_value(&mut self, value: Value) -> XResult<()> {
    //if self.value.r#type() != value.r#type() {
    //return Err(crate::error::XError::new(
    //crate::error::XErrorKind::TagError,
    //"Tag value type mismatch",
    //));
    //}

    //self.value = value;

    //Ok(())
    //}
}
