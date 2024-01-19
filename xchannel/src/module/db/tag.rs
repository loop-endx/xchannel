use serde::{Deserialize, Serialize};
use tracing::trace;

use crate::error::*;

use super::super::tag::Tag as MTag;
use super::super::value::{DataType, Value};
use super::DBLayer;
use super::Record;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub table: String,
    pub name: String,
    pub value: Value,
    pub dtype: DataType,
    pub address: Option<String>,
    pub description: Option<String>,
}

impl From<(&str, &MTag)> for Tag {
    fn from((table, tag): (&str, &MTag)) -> Self {
        Tag {
            table: table.to_string(),
            name: tag.name.clone(),
            value: tag.value.clone(),
            dtype: tag.dtype.clone(),
            address: tag.address.clone(),
            description: tag.description.clone(),
        }
    }
}

impl Tag {
    const TABLE_PREFIX: &'static str = "tag_";

    fn table_name(device: &str) -> String {
        format!("{}{}", Self::TABLE_PREFIX, device)
    }

    pub async fn add(db: &DBLayer, device: &str, table: &str, tags: Vec<&MTag>) -> XResult<()> {
        let tags: Vec<Tag> = tags.iter().map(|t| (table, *t).into()).collect();

        for tag in tags {
            let re: Vec<Record> = db.db.create(Self::table_name(device)).content(tag).await?;
            trace!("store tag {:?}", re);
        }

        Ok(())
    }

    pub async fn delete(db: &DBLayer, device: &str, table: &str, tags: &[String]) -> XResult<()> {
        for tag in tags {
            let re = db
                .db
                .query("DELETE type::table($table) WHERE table = $table_name AND name = $value")
                .bind(("table", Self::table_name(device)))
                .bind(("table_name", table))
                .bind(("value", tag))
                .await?
                .check()?;
            trace!("delete response {:?}", re);
        }
        Ok(())
    }

    pub async fn delete_all(db: &DBLayer, device: &str, table: Option<&str>) -> XResult<()> {
        let re = if let Some(tb) = table {
            db.db
                .query("DELETE type::table($table) WHERE table = $table_name")
                .bind(("table", Self::table_name(device)))
                .bind(("table_name", tb))
                .await?
                .check()?
        } else {
            db.db
                .query("DELETE type::table($table)")
                .bind(("table", Self::table_name(device)))
                .await?
                .check()?
        };
        trace!("delete response {:?}", re);
        Ok(())
    }

    pub async fn select(db: &DBLayer, device: &str, table: &str) -> XResult<Vec<MTag>> {
        let mut re = db
            .db
            .query("SELECT * FROM type::table($table) WHERE table = $table_name")
            .bind(("table", Self::table_name(device)))
            .bind(("table_name", table))
            .await?
            .check()?;

        trace!("load {:?}", re);

        Ok(re.take(0)?)
    }
}
