use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

//0xFF
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum XError {
    #[error("Driver Error: {0}")]
    DriverError(String), // 1001
    #[error("Device Error: {0}")]
    DeviceError(String), // 1002
    #[error("Tag Error: {1} ({0})")]
    TagError(i32, String), // 1003
    #[error("Parameter Error: {0}")]
    ParameterError(String), // 1003
    #[error("DB Error: {0}")]
    DBError(String), // 1101
}

pub enum XErrorKind {
    DriverError,
    DeviceError,
    TagError,
    ParameterError,
    DBError,
}

impl XError {
    pub fn new(kind: XErrorKind, msg: &str) -> XError {
        use XError::*;
        match kind {
            XErrorKind::DeviceError => DeviceError(msg.to_string()),
            XErrorKind::DriverError => DriverError(msg.to_string()),
            XErrorKind::TagError => TagError(-1, msg.to_string()),
            XErrorKind::ParameterError => ParameterError(msg.to_string()),
            XErrorKind::DBError => DBError(msg.to_string()),
        }
    }
}

impl XError {
    pub fn code(&self) -> u64 {
        use XError::*;
        match self {
            DriverError(_) => 1001,
            DeviceError(_) => 1002,
            TagError(_, _) => 1003,
            ParameterError(_) => 1004,
            DBError(_) => 1101,
            //Io(_) => 1101,
        }
    }

    pub fn message(&self) -> String {
        self.to_string()
    }
}

impl From<surrealdb::Error> for XError {
    fn from(err: surrealdb::Error) -> Self {
        XError::new(XErrorKind::DBError, &err.to_string())
    }
}

pub type XResult<T> = std::result::Result<T, XError>;

impl warp::reject::Reject for XError {}
