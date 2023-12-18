use serde_derive::{Deserialize, Serialize};

use crate::error::*;
use crate::module::tag::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub value: Value,
    pub address: Option<String>,
    pub parent: Option<String>,
    pub description: Option<String>,
}

impl Tag {
    pub fn check(tags: &[Tag]) -> XResult<()> {
        use Value::*;
        for (i, tag) in tags.iter().enumerate() {
            match tag.value {
                Base(_) => {
                    if let Some(parent) = &tag.parent {
                        if *parent == tag.name {
                            return Err(TagError::new_with_index(
                                TagErrorKind::Invalid,
                                i as i32 + 1,
                                "Parent can't be self",
                            ));
                        }
                    }
                }
                Array(_) | Struct(_) => {
                    if tag.parent.is_some() {
                        return Err(TagError::new_with_index(
                            TagErrorKind::Invalid,
                            i as i32 + 1,
                            "Array or Struct shouldn't have parent",
                        ));
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::module::tag::BaseValue;

    #[test]
    fn test_encode_tag() {
        let tag = Tag {
            name: "test_name".to_string(),
            value: Value::Base(BaseValue::BOOL(true)),
            address: Some("test_address".to_string()),
            parent: None,
            description: None,
        };

        let json = serde_json::to_string(&tag).unwrap();
        println!("{}", json);
    }
}
