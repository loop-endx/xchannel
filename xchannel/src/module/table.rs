use serde::Serialize;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::error::*;

use super::driver::Parameter;
use super::tag::Tag;

#[derive(Debug)]
pub struct Table {
    name: String,
    description: Option<String>,
    parameter: Parameter,
    tags: Mutex<HashMap<String, Tag>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TableInfo {
    pub name: String,
    pub description: Option<String>,
    pub parameter: Parameter,
}

impl Table {
    pub fn new(name: String, description: Option<String>, parameter: Parameter) -> Self {
        Table {
            name,
            description,
            parameter,
            tags: Mutex::new(HashMap::new()),
        }
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn info(&self) -> TableInfo {
        TableInfo {
            name: self.name.to_string(),
            description: self.description.clone(),
            parameter: self.parameter.clone(),
        }
    }

    pub fn get_tags(&self, name: Option<String>) -> XResult<Vec<Tag>> {
        let tags = self.tags.lock().unwrap();

        let iter = tags.iter();

        if let Some(name) = name {
            Ok(iter
                .filter(|(key, _)| key.contains(&name))
                .map(|(_, tag)| tag.clone())
                .collect())
        } else {
            Ok(iter.map(|(_, tag)| tag.clone()).collect())
        }
    }

    pub fn add_tags(&self, tags: &[Tag]) -> XResult<()> {
        let mut t = self.tags.lock().unwrap();

        for (i, tag) in tags.iter().enumerate() {
            if t.contains_key(&tag.name) {
                return Err(XError::TagError(
                    i as i32 + 1,
                    format!("conflict name {}", tag.name),
                ));
            }

            t.insert(tag.name.clone(), tag.clone());
        }

        Ok(())
    }

    pub fn del_tags(&self, tags: &[String]) -> XResult<()> {
        let mut t = self.tags.lock().unwrap();

        tags.iter().for_each(|tag| {
            t.remove(tag);
        });

        Ok(())
    }
}
