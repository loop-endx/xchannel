use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct DeviceInfo {
    pub name: String,
    pub driver: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TCPClient {
    pub host: String,
    pub port: u16,
    pub timeout: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialClient {
    pub device: String,
    pub baudrate: u32,
    pub parity: String,
    pub stopbits: u8,
    pub databits: u8,
    pub timeout: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddTags {
    pub device: String,
    pub table: String,
    pub tags: Vec<crate::tag::dto::Tag>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DelTags {
    pub device: String,
    pub table: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTables {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub description: Option<String>,
}
