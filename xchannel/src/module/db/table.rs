use serde::{Deserialize, Serialize};
use tracing::trace;

use crate::error::*;

use super::super::driver::Parameter;
use super::tag::Tag;
use super::DBLayer;
use super::Record;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub device: String,
    pub description: Option<String>,
    pub parameter: Parameter,
}

impl Table {
    const TABLE_NAME: &'static str = "table";

    pub async fn select(db: &DBLayer, device: &str) -> XResult<Vec<Table>> {
        let mut re = db
            .db
            .query("SELECT * FROM type::table($table) WHERE device = $device")
            .bind(("table", Self::TABLE_NAME))
            .bind(("device", device))
            .await?
            .check()?;
        trace!("load {:?}", re);
        Ok(re.take(0)?)
    }

    pub async fn add(
        db: &DBLayer,
        name: &str,
        device: &str,
        description: Option<String>,
        parameter: &Parameter,
    ) -> XResult<String> {
        let re: Vec<Record> = db
            .db
            .create(Self::TABLE_NAME)
            .content(Table {
                name: name.to_string(),
                device: device.to_string(),
                description,
                parameter: parameter.clone(),
            })
            .await?;
        trace!("store device {:?}", re);
        if let Some(re) = re.first() {
            Ok(re.id.id.to_string())
        } else {
            Err(XError::DBError(format!("{:?}", re)))
        }
    }

    pub async fn delete(db: &DBLayer, device: &str, name: &str) -> XResult<()> {
        let re = db
            .db
            .query("DELETE type::table($table) WHERE device = $device AND name = $value")
            .bind(("table", Self::TABLE_NAME))
            .bind(("device", device))
            .bind(("value", name))
            .await?
            .check()?;
        Tag::delete_all(db, device, Some(name)).await?;
        trace!("delete response {:?}", re);
        Ok(())
    }

    pub async fn delete_all(db: &DBLayer, device: &str) -> XResult<()> {
        let re = db
            .db
            .query("DELETE type::table($table) WHERE device = $device")
            .bind(("table", Self::TABLE_NAME))
            .bind(("device", device))
            .await?
            .check()?;
        Tag::delete_all(db, device, None).await?;
        trace!("delete response {:?}", re);
        Ok(())
    }
}
