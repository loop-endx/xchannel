use std::fs;

use serde::Deserialize;
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::sql::Thing;
use surrealdb::Surreal;

pub mod device;
pub mod table;

use crate::error::XResult;

#[derive(Debug)]
pub struct DBLayer {
    pub db: Surreal<Db>,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    pub id: Thing,
}

impl DBLayer {
    pub async fn init(&self) -> XResult<()> {
        self.db
            .query(Self::read_sql("xchannel/sql/init.surql")?)
            .await?;

        Ok(())
    }

    pub async fn new() -> XResult<Self> {
        let db = Surreal::new::<RocksDb>("data/xchannel.db").await?;

        db.use_ns("xchannel").use_db("xchannel").await?;

        Ok(Self { db })
    }

    fn read_sql(path: &str) -> XResult<String> {
        let content = fs::read_to_string(path)?;
        Ok(content)
    }
}
