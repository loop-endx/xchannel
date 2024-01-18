use serde_derive::{Deserialize, Serialize};

use super::value::{DataType, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub value: Value,
    pub dtype: DataType,
    pub address: Option<String>,
    pub description: Option<String>,
}
