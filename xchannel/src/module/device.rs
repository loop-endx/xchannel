use serde::Serialize;
use std::collections::HashMap;

use std::sync::Mutex;

use crate::error::*;

use super::driver::{Driver, Parameter, Setting, Tag as DTag};
use super::table::{Table, TableInfo};
use super::tag::Tag;

pub struct Device {
    name: String,
    driver_name: String,
    setting: Option<Setting>,

    driver: Box<dyn Driver + Send>,

    tables: Mutex<HashMap<String, Table>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeviceInfo {
    pub id: String,
    pub name: String,
    pub driver: String,
    pub setting: Option<Setting>,
}

impl Device {
    pub fn new(
        name: &str,
        driver: Box<dyn Driver + Send>,
        setting: &Option<Setting>,
    ) -> XResult<Self> {
        if let Some(setting) = &setting {
            driver.setting(setting)?;
        }

        Ok(Device {
            name: name.to_string(),
            driver_name: driver.info().name.clone(),
            driver,
            setting: setting.clone(),
            tables: Mutex::new(HashMap::new()),
        })
    }

    pub fn info(&self, id: &str) -> DeviceInfo {
        DeviceInfo {
            id: id.to_string(),
            name: self.name.to_string(),
            driver: self.driver_name.to_string(),
            setting: self.setting.clone(),
        }
    }

    pub fn driver_name(&self) -> String {
        self.driver_name.to_string()
    }

    pub fn add_table(
        &self,
        name: &str,
        description: Option<String>,
        parameter: &Parameter,
    ) -> XResult<()> {
        let mut tables = self.tables.lock().unwrap();

        if tables.contains_key(name) {
            return Err(XError::new(
                XErrorKind::TableError,
                &format!("{name} already exists"),
            ));
        }

        self.driver.table_parameter(parameter)?;

        let table = Table::new(name.to_string(), description, parameter.clone());
        tables.insert(name.to_string(), table);

        Ok(())
    }

    pub fn del_table<'a, 'b>(&'b self, name: &'a str) -> XResult<Option<&'a str>> {
        let mut tables = self.tables.lock().unwrap();

        tables
            .remove(name)
            .map_or_else(|| Ok(None), |_| Ok(Some(name)))
    }

    pub fn get_tables(&self, name: Option<String>) -> Vec<TableInfo> {
        let tables = self.tables.lock().unwrap();

        tables
            .iter()
            .filter(|(_, table)| {
                if let Some(n) = &name {
                    table.name().contains(n)
                } else {
                    true
                }
            })
            .map(|(_, table)| table.info())
            .collect()
    }

    pub fn get_tags(&self, table: &str, name: Option<String>) -> XResult<Vec<Tag>> {
        let tables = self.tables.lock().unwrap();

        if let Some(table) = tables.get(table) {
            table.get_tags(name)
        } else {
            Err(XError::new(
                XErrorKind::TableError,
                &format!("{table} not found"),
            ))
        }
    }

    pub fn add_tags(&self, table: &str, tags: &[Tag]) -> XResult<()> {
        let tables = self.tables.lock().unwrap();

        if let Some(table) = tables.get(table) {
            let dtags: Vec<DTag> = tags
                .iter()
                .filter(|tag| tag.address.is_some())
                .map(|tag| tag.into())
                .collect();
            self.driver.tag(&dtags)?;
            table.add_tags(tags)
        } else {
            Err(XError::new(
                XErrorKind::TableError,
                &format!("{table} not found"),
            ))
        }
    }

    pub fn del_tags(&self, table: &str, tags: &[String]) -> XResult<()> {
        let tables = self.tables.lock().unwrap();

        if let Some(table) = tables.get(table) {
            table.del_tags(tags)
        } else {
            Err(XError::new(
                XErrorKind::TableError,
                &format!("{table} not found"),
            ))
        }
    }
}
