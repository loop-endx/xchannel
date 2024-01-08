use serde_derive::{Deserialize, Serialize};

use crate::error::*;

use super::r#type::*;
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
