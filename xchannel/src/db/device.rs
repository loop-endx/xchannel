use serde::{Deserialize, Serialize};

use crate::driver::dto::Parameter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub name: String,
    pub driver: String,
    pub parameters: Vec<Parameter>,
}
