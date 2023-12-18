use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::dto;
use crate::error::*;

use crate::module::tag::Tag;

pub struct Tags {
    pub tags: Mutex<HashMap<String, HashMap<String, Tag>>>,
}

impl Tags {
    pub fn new() -> Arc<Self> {
        Arc::new(Tags {
            tags: Mutex::new(HashMap::new()),
        })
    }

    pub fn add(&self) {
        let mut tags = self.tags.lock().unwrap();

        tags.insert("modbus".to_string(), HashMap::new());
    }

    pub fn insert(&self, driver: String, tags: Vec<dto::tag::Tag>) -> XResult<()> {
        let mut stags = self.tags.lock().unwrap();

        if let Some(driver) = stags.get_mut(&driver) {
            let h_tag: HashMap<&str, &dto::tag::Tag> = tags
                .iter()
                .filter(|tag| !tag.value.is_base())
                .map(|tag| (&tag.name[..], tag))
                .collect();

            for (i, tag) in tags.iter().enumerate() {
                if let Some(parent) = &tag.parent {
                    if driver.get(parent).is_none() {
                        if let Some(t) = h_tag.get(&parent[..]) {
                            if t.value.is_base() {
                                return Err(TagError::new_with_index(
                                    TagErrorKind::Invalid,
                                    i as i32 + 1,
                                    "Parent tag should be array or struct",
                                ));
                            }
                        } else {
                            return Err(TagError::new_with_index(
                                TagErrorKind::Invalid,
                                i as i32 + 1,
                                "Parent tag not found",
                            ));
                        }
                    }
                }
            }

            for tag in tags {
                driver.insert(tag.name.clone(), tag.into());
            }
            Ok(())
        } else {
            Err(DriverError::new(DriverErrorKind::NotFoundDriver, &driver))
        }
    }
}
