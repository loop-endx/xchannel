use crate::error::{XError, XErrorKind, XResult};

use super::super::tag;
use super::super::tag::Value;

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
            Err(XError::new(XErrorKind::TagError, "Tag address is empty"))
        }
    }
}
