use serde::Deserialize;

use crate::driver::dto::Parameter;

#[derive(Debug, Clone, Deserialize)]
pub struct AddDevice {
    pub name: String,
    pub driver: String,
    pub parameters: Vec<Parameter>,
}
