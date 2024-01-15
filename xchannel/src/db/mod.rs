use std::fs;

use serde::{Deserialize, Serialize};
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::sql::Thing;
use surrealdb::Surreal;

pub mod device;

use crate::error::XResult;

pub struct DBLayer {
    db: Surreal<Db>,
}

pub trait DB {
    fn table_name(&self) -> &str;
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
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

    pub async fn create<T>(&self, ele: T) -> XResult<()>
    where
        T: Serialize + DB,
    {
        let _re: Vec<Record> = self.db.create(ele.table_name()).content(ele).await?;
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
