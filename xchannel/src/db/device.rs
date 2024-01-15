use serde::{Deserialize, Serialize};

use crate::driver::dto::Parameter;

use super::DB;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub name: String,
    pub driver: String,
    pub parameters: Vec<Parameter>,
}

impl DB for Device {
    fn table_name(&self) -> &str {
        "device"
    }
}

impl Default for Device {
    fn default() -> Self {
        Device {
            name: "".to_string(),
            driver: "".to_string(),
            parameters: vec![],
        }
    }
}
