pub mod dto;
pub mod table;
pub mod vtype;

use serde_derive::{Deserialize, Serialize};

use crate::error::XResult;

use vtype::*;

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
        let vt: ValueType = tag.dtype.into();

        if let Some(v) = &tag.value {
            if vt != v.v_type() {
                return Err(crate::error::XError::new(
                    crate::error::XErrorKind::TagError,
                    "Tag value type mismatch",
                ));
            }
        }

        Ok(Tag {
            name: tag.name.clone(),
            value: tag.value.clone().unwrap_or(vt.default_value()),
            dtype: tag.dtype.clone(),
            address: tag.address.clone(),
            description: tag.description.clone(),
        })
    }
}
