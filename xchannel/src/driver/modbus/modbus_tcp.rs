use crate::driver::tag::Tag;
use crate::driver::{Driver, DriverInfo};

pub struct ModbusTCP {}

pub struct ModbusTCPSetting {
    host: String,
    port: u16,
    timeout: u16,
}

impl ModbusTCP {
    pub fn new() -> ModbusTCP {
        ModbusTCP {}
    }
}

impl Driver for ModbusTCP {
    type Setting = ModbusTCPSetting;
    type Address = super::Address;

    fn info(&self) -> DriverInfo {
        DriverInfo {
            name: "Modbus TCP".to_string(),
            description: "Modbus TCP is simply the Modbus RTU protocol with a TCP interface that runs on Ethernet.".to_string(),
        }
    }

    fn setting(&self, _setting: Self::Setting) -> Result<(), String> {
        Ok(())
    }

    fn validate(&self, _tags: Vec<Tag<Self::Address>>) -> Result<u16, String> {
        Ok(0)
    }

    fn start(&self) -> Result<(), String> {
        Ok(())
    }

    fn stop(&self) -> Result<(), String> {
        Ok(())
    }

    fn read(&self, _tags: Vec<Tag<Self::Address>>) -> Result<(), String> {
        Ok(())
    }

    fn write(&self, _tags: Vec<Tag<Self::Address>>) -> Result<(), String> {
        Ok(())
    }
}
