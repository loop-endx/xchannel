use std::collections::HashMap;
use std::sync::Mutex;

use crate::error::*;

use super::{dto::Tag as DtoTag, r#type::Value, Tag};

pub struct TagTable<T> {
    name: String,
    description: Option<String>,
    param: T,
    tags: Mutex<HashMap<String, Tag>>,
}

impl<T: Clone> TagTable<T> {
    pub fn new(name: String, description: Option<String>, param: T) -> Self {
        TagTable {
            name,
            description,
            param,
            tags: Mutex::new(HashMap::new()),
        }
    }

    pub fn get_info(&self) -> (String, Option<String>, T) {
        (
            self.name.clone(),
            self.description.clone(),
            self.param.clone(),
        )
    }

    pub fn get_tag(&self, limit: Option<u16>) -> Vec<Tag> {
        let t = self.tags.lock().unwrap();
        let tags = t.iter();

        if let Some(limit) = limit {
            tags.take(limit as usize)
                .map(|(_, tag)| tag.clone())
                .collect()
        } else {
            tags.map(|(_, tag)| tag.clone()).collect()
        }
    }

    pub fn add(&self, tags: &[DtoTag]) -> XResult<()> {
        let mut t = self.tags.lock().unwrap();

        for (i, tag) in tags.iter().enumerate() {
            if t.contains_key(&tag.name) {
                return Err(XError::tag(
                    i as i32 + 1,
                    &format!("conflict name {}", tag.name),
                ));
            }

            let tag: Tag = tag.try_into()?;
            t.insert(tag.name.clone(), tag);
        }

        Ok(())
    }

    pub fn del(&self, tags: &[String]) -> XResult<()> {
        let mut t = self.tags.lock().unwrap();

        for tag in tags {
            t.remove(tag);
        }

        Ok(())
    }

    pub fn update(&self, tags: &[DtoTag]) -> XResult<()> {
        let mut t = self.tags.lock().unwrap();

        for (i, tag) in tags.iter().enumerate() {
            if let Some(t) = t.get_mut(&tag.name) {
                t.update(tag)?;
            } else {
                return Err(XError::tag(
                    i as i32 + 1,
                    &format!("tag {} not found", tag.name),
                ));
            }
        }

        Ok(())
    }

    pub fn update_value(&self, tv: &[(&str, Value)]) -> XResult<()> {
        let mut t = self.tags.lock().unwrap();

        for (name, value) in tv {
            if let Some(t) = t.get_mut(*name) {
                t.update_value(value.clone())?;
            } else {
                return Err(XError::tag(0, &format!("tag {} not found", name)));
            }
        }
        Ok(())
    }
}
