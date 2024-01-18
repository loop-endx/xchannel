use serde::Serialize;
use std::collections::HashMap;
use std::sync::Mutex;

use super::driver::Parameter;
use super::tag::Tag;

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
}
