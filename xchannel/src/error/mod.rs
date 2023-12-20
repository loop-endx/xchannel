use thiserror::Error;

//0xFF
#[derive(Error, Debug)]
pub enum XError {
    #[error("Driver Error: {0}")]
    DriverError(String), // 1001
    #[error("Device Error: {0}")]
    DeviceError(String), // 1002
    #[error("Tag Error: {1} ({0})")]
    TagError(i32, String), // 1003
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error), //1101
}

pub enum XErrorKind {
    DriverError,
    DeviceError,
    TagError,
}

impl XError {
    pub fn new(kind: XErrorKind, msg: &str) -> XError {
        use XError::*;
        match kind {
            XErrorKind::DeviceError => DeviceError(msg.to_string()),
            XErrorKind::DriverError => DriverError(msg.to_string()),
            XErrorKind::TagError => TagError(-1, msg.to_string()),
        }
    }

    pub fn with_index(&self, index: i32) -> XError {
        XError::TagError(index, self.message())
    }

    pub fn tag(index: i32, msg: &str) -> XError {
        XError::TagError(index, msg.to_string())
    }
}

impl XError {
    pub fn code(&self) -> u64 {
        use XError::*;
        match self {
            DriverError(_) => 1001,
            DeviceError(_) => 1002,
            TagError(_, _) => 1003,
            Io(_) => 1101,
        }
    }

    pub fn message(&self) -> String {
        self.to_string()
    }
}

pub type XResult<T> = std::result::Result<T, XError>;

impl warp::reject::Reject for XError {}
