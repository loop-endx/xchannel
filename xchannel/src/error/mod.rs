use std::io;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Tag(TagError),
}

#[derive(Debug)]
pub enum TagError {
    InvalidAddress(&'static str),
    UnsupportType(&'static str),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<TagError> for Error {
    fn from(err: TagError) -> Self {
        Error::Tag(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
pub type TagResult<T> = std::result::Result<T, TagError>;
