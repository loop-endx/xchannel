use serde::{Deserialize, Serialize};
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::sql::Thing;
use surrealdb::Surreal;

pub mod device;

use crate::error::XResult;

pub struct DBLayer {
    db: Surreal<Db>,
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

        Ok(Self { db })
    }

    pub async fn create(&self, tb: &str, ele: impl Serialize) -> XResult<()> {
        let _re: Vec<Record> = self.db.create(tb).content(ele).await?;
        Ok(())
    }

    pub async fn query<T: for<'de> Deserialize<'de>>(&self, tb: &str) -> XResult<Vec<T>> {
        let re: Vec<T> = self.db.select(tb).await?;
        Ok(re)
    }
}
