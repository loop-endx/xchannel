use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::error::{DriverError, DriverErrorKind, XResult};
use crate::module::{
    driver::{Driver, DriverInfo},
    driver::tag::Tag,
};

use crate::drivers::modbus::modbus_tcp::ModbusTcp;

struct Info {
    driver: DriverInfo,
    _validate: fn(Vec<Tag>) -> XResult<()>,
}

pub struct Drivers {
    drivers: Mutex<HashMap<String, Info>>,
}

impl Drivers {
    pub fn new() -> Arc<Self> {
        let drivers = Drivers {
            drivers: Mutex::new(HashMap::new()),
        };

        drivers.register(ModbusTcp::info(), ModbusTcp::validate);

        Arc::new(drivers)
    }

    pub fn get_drivers(&self) -> Vec<DriverInfo> {
        let drivers = self.drivers.lock().unwrap();

        drivers.values().map(|info| info.driver.clone()).collect()
    }

    pub fn _validate<'a: 'static>(&self, name: &'a str, tags: Vec<Tag>) -> XResult<()> {
        let drivers = self.drivers.lock().unwrap();

        if let Some(info) = drivers.get(name) {
            (info._validate)(tags)
        } else {
            Err(DriverError::new(DriverErrorKind::NotFoundDriver, name))
        }
    }

    fn register(&self, info: DriverInfo, validate: fn(Vec<Tag>) -> XResult<()>) {
        let mut drivers = self.drivers.lock().unwrap();

        drivers.insert(
            info.name.to_string(),
            Info {
                driver: info,
                _validate: validate,
            },
        );
    }
}
