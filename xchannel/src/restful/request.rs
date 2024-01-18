use serde::Deserialize;

use crate::module::driver::{Parameter, Setting};

#[derive(Debug, Clone, Deserialize)]
pub struct AddDevice {
    pub name: String,
    pub driver: String,
    pub setting: Option<Setting>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddTable {
    pub name: String,
    pub parameter: Parameter,
    pub description: Option<String>,
}
