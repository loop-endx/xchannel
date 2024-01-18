use serde::{Deserialize, Serialize};
use tracing::info;

use crate::error::*;

use super::super::driver::Parameter;
use super::DBLayer;
use super::Record;

use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub device: Thing,
    pub description: Option<String>,
    pub parameter: Parameter,
}

impl Table {
    const TABLE_NAME: &'static str = "table";

    pub async fn select(db: &DBLayer, device: Thing) -> XResult<Vec<Table>> {
        let mut re = db
            .db
            .query("SELECT * FROM type::table($table) WHERE device = $device")
            .bind(("table", Self::TABLE_NAME))
            .bind(("device", device))
            .await?
            .check()?;
        info!("load {:?}", re);
        Ok(re.take(0)?)
    }

    pub async fn add(
        db: &DBLayer,
        name: &str,
        device: Thing,
        description: Option<String>,
        parameter: &Parameter,
    ) -> XResult<String> {
        let re: Vec<Record> = db
            .db
            .create(Self::TABLE_NAME)
            .content(Table {
                name: name.to_string(),
                device,
                description,
                parameter: parameter.clone(),
            })
            .await?;
        info!("store device {:?}", re);
        if let Some(re) = re.first() {
            Ok(re.id.id.to_string())
        } else {
            Err(XError::DBError(format!("{:?}", re)))
        }
    }
}
