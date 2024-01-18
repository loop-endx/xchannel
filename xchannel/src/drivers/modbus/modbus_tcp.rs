use crate::module::driver::{Driver, DriverInfo, Validate};

use crate::error::XResult;
use crate::module::driver::{Parameter, Setting};

//pub struct Setting {
//pub host: String,
//pub port: u16,
//pub timeout: u16,
//}

//use crate::driver::dto;

pub struct ModbusTcpContext {}

pub struct ModbusTcp {
    pub setting: Option<Setting>,
    pub context: Option<ModbusTcpContext>,
}

impl Default for ModbusTcp {
    fn default() -> Self {
        ModbusTcp {
            setting: None,
            context: None,
        }
    }
}

impl ModbusTcp {
    pub fn new() -> impl Driver {
        ModbusTcp {
            setting: None,
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

    //fn validate(&self, _tags: Vec<crate::tag::Tag>) -> XResult<()> {
    ////for (i, tag) in tags.iter().enumerate() {
    ////if let Err(err::XError) = tag.try_into() {
    ////return Err(err.with_index(i as i32 + 1));
    ////}
    ////}

    //Ok(())
    //}

    fn setting(&self, _parameters: &Setting) -> XResult<()> {
        Ok(())
    }
}

impl Validate for ModbusTcp {
    fn table_parameter(&self, _parameter: &Parameter) -> XResult<()> {
        Ok(())
    }
}
