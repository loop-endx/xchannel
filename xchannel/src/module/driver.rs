use async_trait::async_trait;
use serde_derive::{Deserialize, Serialize};

use crate::error::XResult;

use super::tag::Tag as MTag;
use super::value::{DataType, SimpleValue, Value};

#[derive(Debug, Clone, Serialize)]
pub struct DriverInfo {
    pub name: String,
    pub description: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub option: String,
    pub value: SimpleValue,
}

pub type Setting = Vec<Parameter>;

pub trait Validate {
    fn table_parameter(&self, parameter: &Parameter) -> XResult<()>;
	fn tag(&self, tags: &[Tag]) -> XResult<()>;
}

#[async_trait]
pub trait Driver: Validate {
    fn info(&self) -> DriverInfo;
    fn setting(&self, setting: &Setting) -> XResult<()>;
    //fn validate(&self, tags: Vec<Tag>) -> XResult<()>;
    //fn setting(&self, parameters: &[dto::Parameter]) -> XResult<()>;
}

pub struct Tag {
    pub name: String,
    pub value: Value,
    pub dtype: DataType,
    pub address: String,
}

impl From<&MTag> for Tag {
    fn from(tag: &MTag) -> Self {
        Tag {
            name: tag.name.clone(),
            value: tag.value.clone(),
            dtype: tag.dtype.clone(),
            address: tag.address.clone().unwrap(),
        }
    }
}
