use serde_derive::{Deserialize, Serialize};

use crate::error::*;

use super::vtype::*;
use super::Tag as MTag;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub value: Option<Value>,
    pub dtype: DataType,
    pub address: Option<String>,
    pub description: Option<String>,
}

impl TryFrom<Tag> for MTag {
    type Error = XError;

    fn try_from(tag: Tag) -> XResult<MTag> {
        if let Some(_) = tag.address {
            Ok(MTag::new(&tag)?)
        } else {
            Err(XError::new(XErrorKind::TagError, "Tag address is empty"))
        }
    }
}

impl TryFrom<&Tag> for MTag {
    type Error = XError;

    fn try_from(tag: &Tag) -> XResult<MTag> {
        if let Some(_) = tag.address {
            Ok(MTag::new(tag)?)
        } else {
            Err(XError::new(XErrorKind::TagError, "Tag address is empty"))
        }
    }
}

impl TryFrom<MTag> for Tag {
    type Error = XError;

    fn try_from(tag: MTag) -> XResult<Tag> {
        Ok(Tag {
            name: tag.name,
            value: Some(tag.value),
            dtype: tag.dtype,
            address: tag.address,
            description: tag.description,
        })
    }
}

impl From<&MTag> for Tag {
    fn from(tag: &MTag) -> Tag {
        Tag {
            name: tag.name.clone(),
            value: Some(tag.value.clone()),
            dtype: tag.dtype.clone(),
            address: tag.address.clone(),
            description: tag.description.clone(),
        }
    }
}
