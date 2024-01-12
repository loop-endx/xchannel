use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::drivers::modbus::ModbusParameters;
use crate::error::{XError, XErrorKind, XResult};

use crate::drivers::modbus::modbus_tcp::ModbusTcp;

use super::dto::*;
use super::DriverInfo;
use super::{device, dto};
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

    pub fn add_tags(
        &self,
        device: &str,
        table: &str,
        tags: &[crate::tag::dto::Tag],
    ) -> XResult<()> {
        let mut devices = self.devices.lock().unwrap();

        let device = devices.get_mut(device).ok_or_else(|| {
            XError::new(
                XErrorKind::DeviceError,
                &format!("device not found: {device}"),
            )
        })?;

        match device {
            Device::ModbusTCP(device) => device.add_tags(table, tags)?,
        }

        Ok(())
    }

    pub fn del_tags(&self, device: &str, table: &str, tags: &[String]) -> XResult<()> {
        let mut devices = self.devices.lock().unwrap();

        let device = devices.get_mut(device).ok_or_else(|| {
            XError::new(
                XErrorKind::DeviceError,
                &format!("device not found: {device}"),
            )
        })?;

        match device {
            Device::ModbusTCP(device) => device.del_tags(table, tags)?,
        }

        Ok(())
    }

    pub fn get_tags(&self, device: &str, table: &str, limit: Option<u16>) -> XResult<Vec<crate::tag::dto::Tag>> {
        let devices = self.devices.lock().unwrap();

        let device = devices.get(device).ok_or_else(|| {
            XError::new(
                XErrorKind::DeviceError,
                &format!("device not found: {device}"),
            )
        })?;

        match device {
            Device::ModbusTCP(device) => device.get_tags(table, limit),
        }
    }

    pub fn add_table(
        &self,
        device: &str,
        table: &str,
        description: Option<String>,
        parameters: &[dto::Parameter],
    ) -> XResult<()> {
        let mut devices = self.devices.lock().unwrap();

        let device = devices.get_mut(device).ok_or_else(|| {
            XError::new(
                XErrorKind::DeviceError,
                &format!("device not found: {device}"),
            )
        })?;

        match device {
            Device::ModbusTCP(device) => {
                let mp: ModbusParameters = parameters.try_into()?;
                device.add_table(table.to_string(), description, mp.slave_id)?;
            }
        }

        Ok(())
    }

    pub fn del_table(&self, device: &str, table: &str) -> XResult<()> {
        let mut devices = self.devices.lock().unwrap();

        let device = devices.get_mut(device).ok_or_else(|| {
            XError::new(
                XErrorKind::DeviceError,
                &format!("device not found: {device}"),
            )
        })?;

        match device {
            Device::ModbusTCP(device) => device.del_table(table.to_string())?,
        }

        Ok(())
    }

    pub fn get_tables(&self, device: &str) -> Vec<dto::GetTables> {
        let devices = self.devices.lock().unwrap();

        let device = devices.get(device).unwrap();

        match device {
            Device::ModbusTCP(device) => device.get_tables(),
        }
    }

    fn create_device(&self, driver: &str, parameters: &[dto::Parameter]) -> XResult<Device> {
        match driver {
            "Modbus TCP" => {
                let d = ModbusTcp::new();
                d.setting(parameters)?;

                Ok(Device::ModbusTCP(device::Device::new(Box::new(d))))
            }
            _ => Err(XError::new(
                XErrorKind::DriverError,
                &format!("driver not found: {driver}"),
            )),
        }
    }

    pub fn add_device(
        &self,
        device_name: &str,
        driver_name: &str,
        parameters: &[dto::Parameter],
    ) -> XResult<()> {
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

        let device = self.create_device(driver_name, parameters)?;

        devices.insert(device_name.to_string(), device);

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