use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::dto::device::DeviceInfo;
use crate::error::{XError, XErrorKind, XResult};
use crate::module::driver::{Driver, DriverInfo};

use crate::drivers::modbus::modbus_tcp::ModbusTcp;

enum Device {
    ModbusTCP(ModbusTcp),
}

impl Device {
    fn info(&self) -> DriverInfo {
        match self {
            Device::ModbusTCP(_) => ModbusTcp::info(),
        }
    }
}

pub struct DeviceMgr {
    devices: Mutex<HashMap<String, Device>>,
    drivers: Mutex<HashMap<String, Device>>,
}

impl DeviceMgr {
    pub fn init() -> Arc<Self> {
        let mgr = DeviceMgr {
            devices: Mutex::new(HashMap::new()),
            drivers: Mutex::new(HashMap::new()),
        };

        let mut d = mgr.drivers.lock().unwrap();

        d.insert(
            ModbusTcp::info().name,
            Device::ModbusTCP(ModbusTcp::default()),
        );

        drop(d);
        Arc::new(mgr)
    }

    pub fn get_driver_info(&self) -> Vec<DriverInfo> {
        let d = self.drivers.lock().unwrap();

        d.iter().map(|(_, d)| d.info()).collect()
    }

    pub fn get_device(&self) -> Vec<DeviceInfo> {
        let d = self.devices.lock().unwrap();

        d.iter()
            .map(|(name, d)| DeviceInfo {
                name: name.to_string(),
                driver: d.info().name,
                count: 1,
            })
            .collect()
    }

    pub fn add_device(&self, device_name: &str, driver_name: &str) -> XResult<()> {
        let d = self.drivers.lock().unwrap();

        if !d.contains_key(driver_name) {
            return Err(XError::new(
                XErrorKind::DriverError,
                &format!("driver not found: {driver_name}"),
            ));
        }

        let mut d = self.devices.lock().unwrap();

        if !d.contains_key(device_name) {
            return Err(XError::new(
                XErrorKind::DeviceError,
                &format!("device already exist {device_name}"),
            ));
        }

        d.insert(
            device_name.to_string(),
            Device::ModbusTCP(ModbusTcp::default()),
        );

        Ok(())
    }

    pub fn del_device(&self, device: &str) -> XResult<()> {
        let mut d = self.devices.lock().unwrap();

        d.remove(device).map_or_else(
            || {
                Err(XError::new(
                    XErrorKind::DeviceError,
                    &format!("device not found: {device}"),
                ))
            },
            |_| Ok(()),
        )
    }
}
