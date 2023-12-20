use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct DeviceInfo {
    pub name: String,
    pub driver: String,
    pub count: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddDevice {
    pub name: String,
    pub driver: String,
}
