use serde::{Deserialize, Serialize};

use super::DB;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table<T> {
    pub device: String,
    pub name: String,
    pub description: Option<String>,
    pub param: T,
}

impl<T> DB for Table<T> {
    fn table_name(&self) -> &str {
        "table"
    }

    fn key_name(&self) -> &str {
        "name"
    }
}

impl Default for Table<u8> {
    fn default() -> Self {
        Table {
            device: "".to_string(),
            name: "".to_string(),
            description: None,
            param: 0,
        }
    }
}
