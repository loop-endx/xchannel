use std::fs;

use tracing::{debug, instrument};

use serde::{Deserialize, Serialize};
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::sql::Thing;
use surrealdb::Surreal;

pub mod device;

use crate::error::XResult;

#[derive(Debug)]
pub struct DBLayer {
    db: Surreal<Db>,
}

pub trait DB {
    fn table_name(&self) -> &str;
    fn key_name(&self) -> &str;
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    pub id: Thing,
}

impl DBLayer {
    pub async fn new() -> XResult<Self> {
        let db = Surreal::new::<RocksDb>("data/xchannel.db").await?;

        db.use_ns("xchannel").use_db("xchannel").await?;
        db.query(Self::read_sql("xchannel/sql/init.sql")?).await?;

        Ok(Self { db })
    }

    fn read_sql(path: &str) -> XResult<String> {
        let content = fs::read_to_string(path)?;
        Ok(content)
    }

    pub async fn create<T>(&self, ele: T) -> XResult<Vec<Record>>
    where
        T: Serialize + DB,
    {
        let re: Vec<Record> = self.db.create(ele.table_name()).content(ele).await?;
        Ok(re)
    }

    #[instrument(level = "debug")]
    pub async fn delete<T>(&self, value: &str) -> XResult<()>
    where
        T: for<'de> Deserialize<'de> + DB + Default,
    {
        let t = T::default();
        let name = t.key_name();
        let re = self
            .db
            .query(format!("DELETE type::table($table) WHERE {name} = $value"))
            .bind(("table", t.table_name()))
            .bind(("value", value))
            .await?
            .check()?;
        debug!("delete response {:?}", re);
        Ok(())
    }

    pub async fn query<T>(&self) -> XResult<Vec<T>>
    where
        T: for<'de> Deserialize<'de> + DB + Default,
    {
        let t = T::default();
        let re: Vec<T> = self.db.select(t.table_name()).await?;
        Ok(re)
    }
}
