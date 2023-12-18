use serde_derive::{Deserialize, Serialize};

use crate::dto;
use crate::error::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub value: Value,
    pub address: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Value {
    Base(BaseValue),
    Array(Vec<Tag>),
    Struct(Vec<Tag>),
}

impl Value {
    pub fn is_base(&self) -> bool {
        match self {
            Value::Base(_) => true,
            _ => false,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum BaseValue {
    BIT(u8),
    BOOL(bool),
    UINT8(u8),
    INT8(i8),
    UINT16(u16),
    INT16(i16),
    WORD(u16),
    UINT32(u32),
    INT32(i32),
    FLOAT(f32),
    DWORD(u32),
    UINT64(u64),
    INT64(i64),
    DOUBLE(f64),
    LWORD(u64),
    STRING { length: u16, str: Option<String> },
    BYTES { length: u16, byte: Option<Vec<u8>> },
}

impl<'a> TryFrom<&'a Value> for &'a BaseValue {
    type Error = XError;

    fn try_from(value: &'a Value) -> XResult<Self> {
        match value {
            Value::Base(base_value) => Ok(base_value),
            _ => Err(TagError::new(
                TagErrorKind::UnsupportType,
                "Value is not base type",
            )),
        }
    }
}

impl From<dto::tag::Tag> for Tag {
    fn from(tag: dto::tag::Tag) -> Self {
        Tag {
            name: tag.name,
            value: tag.value,
            address: tag.address,
            description: tag.description,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_tag() {
        let tag = Tag {
            name: "test_name".to_string(),
            value: Value::Base(BaseValue::BOOL(true)),
            address: Some("test_address".to_string()),
            description: None,
        };

        let json = serde_json::to_string(&tag).unwrap();
        println!("{}", json);
    }
}
