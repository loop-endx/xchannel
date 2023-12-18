use async_trait::async_trait;
use serde_derive::Serialize;

use crate::error::XResult;

pub mod tag;

#[derive(Debug, Clone, Serialize)]
pub struct DriverInfo {
    pub name: String,
    pub description: String,
    pub version: String,
}

//struct _Context<T> {
//context: T,
//tags: Vec<tag::Tag>,
//}

#[async_trait]
pub trait Driver {
    //    type Context;

    fn new(setting: &str) -> Self;
    fn info() -> DriverInfo;
    fn validate(tags: Vec<tag::Tag>) -> XResult<()>;

    //async fn setting(&self, setting: Self::Setting) -> Result<(), Error>;
    //async fn start(&self) -> Result<(), Error>;
    //async fn stop(&self) -> Result<(), Error>;

    //async fn scan(&self, &mut context: Self::Context) -> Result<Vec<Tag>, Error>;
    //async fn write(&self, &context: Self::Context) -> Result<(), Error>;
}
