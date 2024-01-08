use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::error::{XError, XErrorKind, XResult};

use crate::drivers::modbus::modbus_tcp::ModbusTcp;

use super::device;
use super::dto::*;
use super::DriverInfo;
use crate::driver::Driver;

enum Device {
    ModbusTCP(device::Device<u8>),
}

pub struct DeviceMgr {
    devices: Mutex<HashMap<String, Device>>,
    drivers: HashMap<String, DriverInfo>,
}

impl DeviceMgr {
    pub fn init() -> Arc<Self> {
        let mut mgr = DeviceMgr {
            devices: Mutex::new(HashMap::new()),
            drivers: HashMap::new(),
        };

        mgr.drivers.insert(
            ModbusTcp::default().info().name,
            ModbusTcp::default().info(),
        );

        Arc::new(mgr)
    }

    pub fn get_drivers(&self) -> Vec<DriverInfo> {
        self.drivers.values().cloned().collect()
    }

    pub fn add_device(&self, device_name: &str, driver_name: &str) -> XResult<()> {
        if !self.drivers.contains_key(driver_name) {
            return Err(XError::new(
                XErrorKind::DriverError,
                &format!("driver not found: {driver_name}"),
            ));
        }
        let mut devices = self.devices.lock().unwrap();

        if devices.contains_key(device_name) {
            return Err(XError::new(
                XErrorKind::DeviceError,
                &format!("device already exist {device_name}"),
            ));
        }

        match driver_name {
            "Modbus TCP" => {
                devices.insert(
                    device_name.to_string(),
                    Device::ModbusTCP(device::Device::new(
                        device_name.to_string(),
                        Box::new(ModbusTcp::new(None)),
                    )),
                );
            }
            _ => {
                return Err(XError::new(
                    XErrorKind::DriverError,
                    &format!("driver not found: {driver_name}"),
                ));
            }
        }
        Ok(())
    }

    pub fn del_device(&self, device: &str) -> XResult<()> {
        let mut devices = self.devices.lock().unwrap();

        devices.remove(device).map_or_else(
            || {
                Err(XError::new(
                    XErrorKind::DeviceError,
                    &format!("device not found: {device}"),
                ))
            },
            |_| Ok(()),
        )
    }

    pub fn get_devices(&self, driver: Option<String>) -> Vec<DeviceInfo> {
        let devices = self.devices.lock().unwrap();

        devices
            .iter()
            .filter(|(_, device)| {
                if let Some(driver) = &driver {
                    match device {
                        Device::ModbusTCP(device) => device.get_driver() == *driver,
                    }
                } else {
                    true
                }
            })
            .map(|(name, device)| match device {
                Device::ModbusTCP(device) => DeviceInfo {
                    name: name.clone(),
                    driver: device.get_driver(),
                },
            })
            .collect()
    }
}
