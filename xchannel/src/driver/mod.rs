use async_trait::async_trait;
use serde_derive::Serialize;

use crate::error::XResult;
use crate::tag::Tag;

pub mod device;
pub mod dto;
pub mod mgr;
pub mod tag;

#[derive(Debug, Clone, Serialize)]
pub struct DriverInfo {
    pub name: String,
    pub description: String,
    pub version: String,
}

#[async_trait]
pub trait Driver {
    fn info(&self) -> DriverInfo;
    fn validate(&self, tags: Vec<Tag>) -> XResult<()>;
    // fn setting(&self) -> XResult<()>;
}
