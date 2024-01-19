use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

//0xFF
#[derive(Error, Clone, Debug, Serialize, Deserialize)]
pub enum XError {
    #[error("Driver Error: {0}")]
    DriverError(String), // 1001
    #[error("Device Error: {0}")]
    DeviceError(String), // 1002
    #[error("Table Error: {0}")]
    TableError(String), // 1003
    #[error("Tag Error: {1} ({0})")]
    TagError(i32, String), // 1004
    #[error("Parameter Error: {0}")]
    ParameterError(String), // 1005
    #[error("DB Error: {0}")]
    DBError(String), // 1101
    #[error("Other Error: {0}")]
    IOError(String), // 1201
}

#[derive(Debug, PartialEq)]
pub enum XErrorKind {
    DriverError,
    DeviceError,
    TableError,
    TagError,
    ParameterError,
    DBError,
    IOError,
}

pub type XResult<T> = std::result::Result<T, XError>;

impl XError {
    pub fn new(kind: XErrorKind, msg: &str) -> XError {
        use XError::*;
        match kind {
            XErrorKind::DriverError => DriverError(msg.to_string()),
            XErrorKind::DeviceError => DeviceError(msg.to_string()),
            XErrorKind::TableError => TableError(msg.to_string()),
            XErrorKind::TagError => TagError(-1, msg.to_string()),
            XErrorKind::ParameterError => ParameterError(msg.to_string()),
            XErrorKind::DBError => DBError(msg.to_string()),
            XErrorKind::IOError => IOError(msg.to_string()),
        }
    }

    pub fn code(&self) -> i64 {
        use XError::*;
        match self {
            DriverError(_) => 1001,
            DeviceError(_) => 1002,
            TableError(_) => 1003,
            TagError(_, _) => 1004,
            ParameterError(_) => 1005,
            DBError(_) => 1101,
            IOError(_) => 1201,
        }
    }

    pub fn message(&self) -> String {
        self.to_string()
    }

    pub fn with_index(&self, index: i32) -> XError {
        use XError::*;
        match self {
            TagError(_, msg) => TagError(index, msg.to_string()),
            _ => self.clone(),
        }
    }

    pub fn get_index(&self) -> i32 {
        use XError::*;
        match self {
            TagError(index, _) => *index,
            _ => -1,
        }
    }

    pub fn kind(&self) -> XErrorKind {
        use XError::*;
        match self {
            DriverError(_) => XErrorKind::DriverError,
            DeviceError(_) => XErrorKind::DeviceError,
            TableError(_) => XErrorKind::TableError,
            TagError(_, _) => XErrorKind::TagError,
            ParameterError(_) => XErrorKind::ParameterError,
            DBError(_) => XErrorKind::DBError,
            IOError(_) => XErrorKind::IOError,
        }
    }
}

impl From<surrealdb::Error> for XError {
    fn from(err: surrealdb::Error) -> Self {
        XError::new(XErrorKind::DBError, &err.to_string())
    }
}

impl From<std::io::Error> for XError {
    fn from(err: std::io::Error) -> Self {
        XError::new(XErrorKind::IOError, &err.to_string())
    }
}

impl warp::reject::Reject for XError {}
