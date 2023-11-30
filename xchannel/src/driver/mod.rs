use std::io::Error;

use async_trait::async_trait;
use serde::ser::SerializeStruct;
use serde::Serialize;

use crate::tag::Tag;

pub mod modbus;

pub struct DriverInfo<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub version: &'a str,
}

impl<'a> Serialize for DriverInfo<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("DriverInfo", 2)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("description", &self.description)?;
        state.serialize_field("version", &self.version)?;
        state.end()
    }
}

struct Context<T, S> {
    context: T,
    tags: Vec<Tag<S>>,
}

#[async_trait]
pub trait Driver<S> {
    type Setting;
    type Address;
    type Context;

    fn new() -> Self;
    fn info() -> DriverInfo<'static>;
    fn validate(tags: Vec<Tag<S>>) -> Result<(), Error>;

    async fn setting(&self, setting: Self::Setting) -> Result<(), Error>;
    async fn start(&self) -> Result<(), Error>;
    async fn stop(&self) -> Result<(), Error>;

    async fn scan(&self, &mut context: Self::Context) -> Result<Vec<Tag<S>>, Error>;
    async fn write(&self, &context: Self::Context) -> Result<(), Error>;
}
