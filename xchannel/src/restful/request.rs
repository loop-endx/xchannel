use serde::Deserialize;

use crate::error::*;

use crate::module::{
    driver::{Parameter, Setting},
    tag::Tag,
    value::*,
};

#[derive(Debug, Clone, Deserialize)]
pub struct AddDevice {
    pub name: String,
    pub driver: String,
    pub setting: Option<Setting>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddTable {
    pub name: String,
    pub parameter: Parameter,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddTag {
    pub name: String,
    pub value: Option<Value>,
    pub dtype: DataType,
    pub address: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DelTag {
    pub name: String,
}

impl TryFrom<&AddTag> for Tag {
    type Error = XError;

    fn try_from(tag: &AddTag) -> XResult<Self> {
        let vtype: ValueType = tag.dtype.into();

        if let Some(v) = &tag.value {
            if vtype != v.v_type() {
                return Err(XError::new(XErrorKind::TagError, "Tag value type mismatch"));
            }
        }

        Ok(Tag {
            name: tag.name.clone(),
            value: tag.value.clone().unwrap_or(vtype.default_value()),
            dtype: tag.dtype,
            address: tag.address.clone(),
            description: tag.description.clone(),
        })
    }
}
