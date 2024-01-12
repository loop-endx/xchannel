use std::collections::HashMap;
use std::sync::Mutex;

use crate::error::*;

use crate::tag::{dto::Tag as DtoTag, table::TagTable};

use super::dto::Parameter;
use super::{dto, Driver};

pub struct Device<T> {
    driver: Box<dyn Driver + Send>,

    tables: Mutex<HashMap<String, TagTable<T>>>,
}

impl<T: Clone> Device<T> {
    pub fn new(driver: Box<dyn Driver + Send>) -> Self {
        Device {
            driver,
            tables: Mutex::new(HashMap::new()),
        }
    }

    pub fn get_driver(&self) -> String {
        self.driver.info().name
    }

    pub fn add_table(&self, name: String, description: Option<String>, param: T) -> XResult<()> {
        let mut t = self.tables.lock().unwrap();

        if t.contains_key(&name) {
            return Err(XError::new(
                XErrorKind::TagError,
                &format!("conflict name {}", name),
            ));
        }

        t.insert(name.clone(), TagTable::new(name, description, param));
        Ok(())
    }

    pub fn del_table(&self, name: String) -> XResult<()> {
        let mut t = self.tables.lock().unwrap();

        t.remove(&name);
        Ok(())
    }

    pub fn get_tables(&self) -> Vec<dto::GetTables> {
        let t = self.tables.lock().unwrap();

        t.iter()
            .map(|(_, table)| {
                let (name, description, _param) = table.get_info();
                dto::GetTables {
                    name,
                    parameters: vec![Parameter {
                        name: "param".to_string(),
                        value: "11".to_string(),
                    }],
                    description,
                }
            })
            .collect()
    }

    pub fn add_tags(&self, table: &str, tags: &[DtoTag]) -> XResult<()> {
        let t = self.tables.lock().unwrap();

        if let Some(table) = t.get(table) {
            table.add(tags)
        } else {
            Err(XError::new(
                XErrorKind::TagError,
                &format!("table {} not found", table),
            ))
        }
    }

    pub fn del_tags(&self, table: &str, tags: &[String]) -> XResult<()> {
        let t = self.tables.lock().unwrap();

        if let Some(table) = t.get(table) {
            table.del(tags)
        } else {
            Err(XError::new(
                XErrorKind::TagError,
                &format!("table {} not found", table),
            ))
        }
    }

    pub fn get_tags(&self, table: &str, limit: Option<u16>) -> XResult<Vec<DtoTag>> {
        let t = self.tables.lock().unwrap();

        if let Some(table) = t.get(table) {
            Ok(table.get_tag(limit))
        } else {
            Err(XError::new(
                XErrorKind::TagError,
                &format!("table {} not found", table),
            ))
        }
    }
}
