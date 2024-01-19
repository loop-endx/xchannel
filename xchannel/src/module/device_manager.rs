use std::collections::HashMap;
use std::sync::Arc;

use log::warn;
use tokio::sync::Mutex;

use crate::drivers::modbus::modbus_tcp::ModbusTcp;
use crate::error::*;

use super::db;
use super::db::device::Device as DBDevice;
use super::db::table::Table as DBTable;
use super::db::tag::Tag as DBTag;
use super::device::Device;
use super::device::DeviceInfo;
use super::driver::DriverInfo;
use super::driver::{Driver, Parameter, Setting};
use super::table::TableInfo;
use super::tag::Tag;

pub struct DeviceMgr {
    devices: Mutex<HashMap<String, (String, Device)>>,
    ids: Mutex<HashMap<String, String>>,
    drivers: HashMap<String, DriverInfo>,
    db: db::DBLayer,
}

impl DeviceMgr {
    pub async fn init() -> XResult<Arc<Self>> {
        let db = db::DBLayer::new().await?;
        db.init().await?;
        let mut mgr = DeviceMgr {
            devices: Mutex::new(HashMap::new()),
            ids: Mutex::new(HashMap::new()),
            drivers: HashMap::new(),
            db,
        };

        mgr.drivers.insert(
            ModbusTcp::default().info().name,
            ModbusTcp::default().info(),
        );

        let mgr = Arc::new(mgr);
        mgr.load().await?;

        Ok(mgr)
    }

    pub fn get_drivers(&self) -> Vec<DriverInfo> {
        self.drivers.values().cloned().collect()
    }

    // name, id, driver
    pub async fn get_devices(
        &self,
        query: (Option<String>, Option<String>, Option<String>),
    ) -> Vec<DeviceInfo> {
        let devices = self.devices.lock().await;

        devices
            .iter()
            .filter(|(key, (id, device))| {
                if let Some(name) = &query.0 {
                    *key == name
                } else if let Some(idd) = &query.1 {
                    *idd == *id
                } else if let Some(driver) = &query.2 {
                    device.driver_name() == *driver
                } else {
                    true
                }
            })
            .map(|(_, (id, device))| device.info(id))
            .collect()
    }

    pub async fn add_device(
        &self,
        name: &str,
        driver: &str,
        setting: &Option<Setting>,
    ) -> XResult<String> {
        if !self.drivers.contains_key(driver) {
            return Err(XError::new(
                XErrorKind::DriverError,
                &format!("{driver} not found"),
            ));
        }

        let mut devices = self.devices.lock().await;

        if devices.contains_key(name) {
            return Err(XError::new(
                XErrorKind::DriverError,
                &format!("{name} already exists"),
            ));
        }

        let device = self.create_device(name, driver, setting)?;

        let id = DBDevice::add(
            &self.db,
            &DBDevice {
                id: None,
                name: name.to_string(),
                driver: driver.to_string(),
                setting: setting.clone(),
            },
        )
        .await?;

        devices.insert(name.to_string(), (id.clone(), device));
        let mut ids = self.ids.lock().await;
        ids.insert(id.clone(), name.to_string());

        Ok(id)
    }

    pub async fn del_device<'a>(&'a self, name: &'a str) -> XResult<Option<&'a str>> {
        let mut devices = self.devices.lock().await;

        if let Err(err) = DBDevice::delete(&self.db, name).await {
            warn!("delete {name} from db, {}", err.to_string());
        }

        devices
            .remove(name)
            .map_or_else(|| Ok(None), |_| Ok(Some(name)))
    }

    pub async fn get_tables(&self, device: &str, name: Option<String>) -> XResult<Vec<TableInfo>> {
        let devices = self.devices.lock().await;

        if !devices.contains_key(device) {
            return Err(XError::new(
                XErrorKind::DeviceError,
                &format!("{device} not found"),
            ));
        }

        let (_, device) = devices.get(device).unwrap();

        Ok(device.get_tables(name))
    }

    pub async fn add_table(
        &self,
        device: &str,
        name: &str,
        description: Option<String>,
        param: &Parameter,
    ) -> XResult<()> {
        let devices = self.devices.lock().await;

        if !devices.contains_key(device) {
            return Err(XError::new(
                XErrorKind::DeviceError,
                &format!("{device} not found"),
            ));
        }

        let (_, dev) = devices.get(device).unwrap();

        dev.add_table(name, description.clone(), param)?;

        DBTable::add(&self.db, name, device, description, param).await?;

        Ok(())
    }

    pub async fn del_table<'a>(
        &'a self,
        device: &'a str,
        name: &'a str,
    ) -> XResult<Option<&'a str>> {
        let devices = self.devices.lock().await;

        if let Some((_, dev)) = devices.get(device) {
            let re = dev.del_table(name);
            DBTable::delete(&self.db, device, name).await?;

            re
        } else {
            Err(XError::new(
                XErrorKind::DeviceError,
                &format!("{device} not found"),
            ))
        }
    }

    pub async fn get_tags(
        &self,
        device: &str,
        table: &str,
        name: Option<String>,
    ) -> XResult<Vec<Tag>> {
        let devices = self.devices.lock().await;

        if let Some((_, dev)) = devices.get(device) {
            dev.get_tags(table, name)
        } else {
            Err(XError::new(
                XErrorKind::DeviceError,
                &format!("{device} not found"),
            ))
        }
    }

    pub async fn add_tags(&self, device: &str, table: &str, tags: Vec<Tag>) -> XResult<()> {
        let devices = self.devices.lock().await;

        if let Some((_, dev)) = devices.get(device) {
            let result = dev.add_tags(table, &tags);
            let mut index = tags.len() as i32;

            if let Err(err) = &result {
                if err.kind() != XErrorKind::TagError {
                    return Err(err.clone());
                } else {
                    index = err.get_index() - 1;
                }
            }
            DBTag::add(
                &self.db,
                device,
                table,
                tags.iter().take(index as usize).collect(),
            )
            .await?;
            result
        } else {
            Err(XError::new(
                XErrorKind::DeviceError,
                &format!("{device} not found"),
            ))
        }
    }

    pub async fn del_tags(&self, device: &str, table: &str, tags: Vec<String>) -> XResult<()> {
        let devices = self.devices.lock().await;

        if let Some((_, dev)) = devices.get(device) {
            let _ = dev.del_tags(table, &tags);
            DBTag::delete(&self.db, device, table, &tags).await
        } else {
            Err(XError::new(
                XErrorKind::DeviceError,
                &format!("{device} not found"),
            ))
        }
    }

    fn create_device(
        &self,
        name: &str,
        driver: &str,
        setting: &Option<Setting>,
    ) -> XResult<Device> {
        match driver {
            "Modbus TCP" => {
                let d = ModbusTcp::new();
                if let Some(setting) = setting {
                    d.setting(setting)?;
                }
                let device = Device::new(name, Box::new(d), setting)?;
                Ok(device)
            }
            _ => Err(XError::new(
                XErrorKind::DriverError,
                &format!("driver not found: {driver}"),
            )),
        }
    }

    async fn load(&self) -> XResult<()> {
        let mut devices = self.devices.lock().await;
        let mut ids = self.ids.lock().await;
        let de = DBDevice::select(&self.db).await?;

        for device in de {
            let d = self.create_device(&device.name, &device.driver, &device.setting)?;
            let id = device.id.unwrap().id.to_string();

            let tables = DBTable::select(&self.db, &device.name).await?;
            for table in tables {
                d.add_table(&table.name, table.description, &table.parameter)?;

                let tags = DBTag::select(&self.db, &device.name, &table.name).await;

                if let Ok(tags) = tags {
                    d.add_tags(&table.name, &tags)?;
                }
            }

            devices.insert(device.name.to_string(), (id.clone(), d));
            ids.insert(id, device.name);
        }

        Ok(())
    }
}
