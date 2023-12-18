use crate::error::{TagError, TagErrorKind, XError, XResult};

use super::super::tag::Value;
use super::super::tag;

#[derive(Debug, Clone)]
pub struct Tag {
    pub name: String,
    pub value: Value,
    pub address: String,
}

impl TryFrom<tag::Tag> for Tag {
    type Error = XError;

    fn try_from(tag: tag::Tag) -> XResult<Self> {
        if let Some(address) = tag.address {
            Ok(Tag {
                name: tag.name,
                value: tag.value,
                address,
            })
        } else {
            Err(TagError::new(TagErrorKind::Invalid, "Tag address is empty"))
        }
    }
}
