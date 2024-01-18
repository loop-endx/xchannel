use serde::{Deserialize, Serialize};
use tracing::{info, trace};

use crate::error::*;

use super::super::driver::Setting;
use super::DBLayer;
use super::Record;

use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub id: Option<Thing>,
    pub name: String,
    pub driver: String,
    pub setting: Option<Setting>,
}

impl Device {
    const TABLE_NAME: &'static str = "device";

    pub fn record_link(id: &str) -> Thing {
        Thing::from((Self::TABLE_NAME, id))
    }

    pub async fn select(db: &DBLayer) -> XResult<Vec<Device>> {
        let re = db.db.select(Self::TABLE_NAME).await?;
        trace!("load {:?}", re);
        Ok(re)
    }

    pub async fn add(db: &DBLayer, device: &Device) -> XResult<String> {
        let re: Vec<Record> = db.db.create(Self::TABLE_NAME).content(device).await?;
        info!("store device {:?}", re);
        if let Some(re) = re.first() {
            Ok(re.id.id.to_string())
        } else {
            Err(XError::DBError(format!("{:?}", re)))
        }
    }

    pub async fn delete(db: &DBLayer, name: &str) -> XResult<()> {
        let re = db
            .db
            .query("DELETE type::table($table) WHERE name = $value")
            .bind(("table", Self::TABLE_NAME))
            .bind(("value", name))
            .await?
            .check()?;
        trace!("delete response {:?}", re);
        Ok(())
    }
}
