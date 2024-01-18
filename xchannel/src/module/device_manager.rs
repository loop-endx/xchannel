use std::collections::HashMap;
use std::sync::Arc;

use log::warn;
use tokio::sync::Mutex;

use crate::drivers::modbus::modbus_tcp::ModbusTcp;
use crate::error::*;

use super::db;
use super::db::device::Device as DBDevice;
use super::db::table::Table as DBTable;
use super::device::Device;
use super::device::DeviceInfo;
use super::driver::DriverInfo;
use super::driver::{Driver, Parameter, Setting};
use super::table::TableInfo;

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

        let (id, device) = devices.get(device).unwrap();

        device.add_table(name, description.clone(), param)?;

        DBTable::add(
            &self.db,
            name,
            DBDevice::record_link(id),
            description,
            param,
        )
        .await?;

        Ok(())
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

            let tables = DBTable::select(&self.db, DBDevice::record_link(&id)).await?;
            for table in tables {
                d.add_table(&table.name, table.description, &table.parameter)?;
            }

            devices.insert(device.name.to_string(), (id.clone(), d));
            ids.insert(id, device.name);
        }

        Ok(())
    }
}
