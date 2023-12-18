use thiserror::Error;

//0xFF
#[derive(Error, Debug)]
pub enum XError {
    #[error("Driver Error: {0}")]
    Driver(#[from] DriverError), // 1001
    #[error("Tag Error: {0}")]
    Tag(#[from] TagError), // 1010
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error), //1101
}

#[derive(Clone, Error, Debug)]
pub enum DriverError {
    #[error("not found driver: {0}")]
    NotFoundDriver(String), // 100101
}

#[derive(Clone, Error, Debug)]
pub enum TagError {
    #[error("Invalid Tag({0}): {1}")]
    Invalid(i32, String), // 101001
    #[error("Invalid Address({0}): {1}")]
    InvalidAddress(i32, String), // 101002
    #[error("Unsupport Type({0}): {1}")]
    UnsupportType(i32, String), // 101103
}

pub enum TagErrorKind {
    Invalid,
    InvalidAddress,
    UnsupportType,
}

impl XError {
    pub fn with_index(self, index: i32) -> XError {
        match self {
            XError::Tag(err) => err.with_index(index),
            _ => self,
        }
    }
}

impl TagError {
    pub fn new(kind: TagErrorKind, msg: &str) -> XError {
        match kind {
            TagErrorKind::Invalid => XError::Tag(TagError::Invalid(-1, msg.to_string())),
            TagErrorKind::InvalidAddress => {
                XError::Tag(TagError::InvalidAddress(-1, msg.to_string()))
            }
            TagErrorKind::UnsupportType => {
                XError::Tag(TagError::UnsupportType(-1, msg.to_string()))
            }
        }
    }

    pub fn with_index(&self, index: i32) -> XError {
        match self {
            TagError::Invalid(_, msg) => XError::Tag(TagError::Invalid(index, msg.to_string())),
            TagError::InvalidAddress(_, msg) => {
                XError::Tag(TagError::InvalidAddress(index, msg.to_string()))
            }
            TagError::UnsupportType(_, msg) => {
                XError::Tag(TagError::UnsupportType(index, msg.to_string()))
            }
        }
    }

    pub fn new_with_index(kind: TagErrorKind, index: i32, msg: &str) -> XError {
        match kind {
            TagErrorKind::Invalid => XError::Tag(TagError::Invalid(index, msg.to_string())),
            TagErrorKind::InvalidAddress => {
                XError::Tag(TagError::InvalidAddress(index, msg.to_string()))
            }
            TagErrorKind::UnsupportType => {
                XError::Tag(TagError::UnsupportType(index, msg.to_string()))
            }
        }
    }
}

pub enum DriverErrorKind {
    NotFoundDriver,
}

impl DriverError {
    pub fn new(kind: DriverErrorKind, msg: &str) -> XError {
        match kind {
            DriverErrorKind::NotFoundDriver => {
                XError::Driver(DriverError::NotFoundDriver(msg.to_string()))
            }
        }
    }
}

impl XError {
    pub fn code(&self) -> u64 {
        match self {
            XError::Driver(err) => err.code(),
            XError::Tag(err) => err.code(),
            XError::Io(_) => 110100,
        }
    }

    pub fn message(&self) -> String {
        self.to_string()
    }
}

impl TagError {
    pub fn code(&self) -> u64 {
        match self {
            TagError::Invalid(_, _) => 101001,
            TagError::InvalidAddress(_, _) => 101002,
            TagError::UnsupportType(_, _) => 101003,
        }
    }
}

impl DriverError {
    pub fn code(&self) -> u64 {
        match self {
            DriverError::NotFoundDriver(_) => 100101,
        }
    }
}

pub type XResult<T> = std::result::Result<T, XError>;

impl warp::reject::Reject for XError {}
