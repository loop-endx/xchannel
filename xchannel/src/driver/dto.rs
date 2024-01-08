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
pub enum Client {
    TCP(TCPClient),
    Serial(SerialClient),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddDevice {
    pub name: String,
    pub driver: String,
    pub target: Option<Client>,
    pub parameters: Vec<Parameter>,
}
