use crate::module::driver::{tag::Tag, Driver, DriverInfo};

use crate::error::XResult;

use super::super::modbus::Address;

pub struct _ModbusTcpSetting {
    pub host: String,
    pub port: u16,
}

pub struct ModbusTcpContext {}

pub struct ModbusTcp {
    pub setting: String,
    pub context: Option<ModbusTcpContext>,
}

impl Driver for ModbusTcp {
    //type Setting = ModbusTcpSetting;
    //type Context = ModbusTcpContext;

    fn new(setting: &str) -> Self {
        ModbusTcp {
            setting: setting.to_string(),
            context: None,
        }
    }

    fn info() -> DriverInfo {
        DriverInfo {
            name: "Modbus TCP".to_string(),
            description: "Modbus TCP description".to_string(),
            version: "0.1.0".to_string(),
        }
    }

    fn validate(tags: Vec<Tag>) -> XResult<()> {
        for (i, tag) in tags.iter().enumerate() {
            if let Err(err) = TryInto::<Address>::try_into(tag) {
                return Err(err.with_index(i as i32 + 1));
            }
        }

        Ok(())
    }
}
