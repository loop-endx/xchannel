use crate::driver::{Driver, DriverInfo};

use crate::error::XResult;

pub struct _ModbusTcpSetting {
    pub host: String,
    pub port: u16,
}

pub struct ModbusTcpContext {}

pub struct ModbusTcp {
    pub setting: String,
    pub context: Option<ModbusTcpContext>,
}

impl Default for ModbusTcp {
    fn default() -> Self {
        ModbusTcp {
            setting: "".to_string(),
            context: None,
        }
    }
}

impl ModbusTcp {
    pub fn new(setting: Option<&str>) -> impl Driver {
        ModbusTcp {
            setting: setting.unwrap_or("").to_string(),
            context: None,
        }
    }
}

impl Driver for ModbusTcp {
    fn info(&self) -> DriverInfo {
        DriverInfo {
            name: "Modbus TCP".to_string(),
            description: "Modbus TCP description".to_string(),
            version: "0.1.0".to_string(),
        }
    }

    fn validate(&self, _tags: Vec<crate::tag::Tag>) -> XResult<()> {
        //for (i, tag) in tags.iter().enumerate() {
        //if let Err(err::XError) = tag.try_into() {
        //return Err(err.with_index(i as i32 + 1));
        //}
        //}

        Ok(())
    }
}
