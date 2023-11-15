pub mod drivers;
mod tag;

use serde::ser::SerializeStruct;
use serde::Serialize;
pub struct DriverInfo {
    pub name: String,
    pub description: String,
}

impl Serialize for DriverInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("DriverInfo", 2)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("description", &self.description)?;
        state.end()
    }
}

pub trait Driver {
    type Setting;

    fn info(&self) -> DriverInfo;
    fn setting(&self, setting: Self::Setting) -> Result<(), String>;
    fn start(&self) -> Result<(), String>;
    fn stop(&self) -> Result<(), String>;
    fn read(&self, tags: Vec<tag::Tag>) -> Result<(), String>;
    fn write(&self, tags: Vec<tag::Tag>) -> Result<(), String>;
}
