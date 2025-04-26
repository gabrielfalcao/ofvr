use std::fmt::Display;
use std::num::ParseIntError;
use std::string::FromUtf8Error;
use serde::ser::{Serialize, SerializeStruct, Serializer};

use hex::FromHexError;

#[derive(Debug, Clone)]
pub enum Error {
    IOCoreError(String),
    IOError(String),
    InvalidUtf8(String),
    ParseIntError(String),
    FileSystemError(String),
    ComparisonError(String),
    DecodingError(String),
    EncodingError(String),
}

impl Serialize for Error {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Error", 2)?;
        s.serialize_field("variant", &self.variant())?;
        s.serialize_field("message", &format!("{}", self))?;
        s.end()
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::IOCoreError(e) => format!("{}", e),
                Self::InvalidUtf8(e) => e.to_string(),
                Self::IOError(e) => e.to_string(),
                Self::FileSystemError(e) => e.to_string(),
                Self::ComparisonError(e) => e.to_string(),
                Self::ParseIntError(e) => e.to_string(),
                Self::DecodingError(e) => e.to_string(),
                Self::EncodingError(e) => e.to_string(),
            }
        )
    }
}

impl Error {
    pub fn variant(&self) -> String {
        match self {
            Error::IOCoreError(_) => "IOCoreError",
            Error::InvalidUtf8(_) => "InvalidUtf8",
            Error::IOError(_) => "IOError",
            Error::FileSystemError(_) => "FileSystemError",
            Error::ComparisonError(_) => "ComparisonError",
            Error::ParseIntError(_) => "ParseIntError",
            Error::DecodingError(_) => "DecodingError",
            Error::EncodingError(_) => "EncodingError",
        }
        .to_string()
    }
}

impl std::error::Error for Error {}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Error::ParseIntError(format!("{}", e))
    }
}
impl From<iocore::Error> for Error {
    fn from(e: iocore::Error) -> Self {
        Error::IOCoreError(format!("{}", e))
    }
}
impl From<Box<bincode::ErrorKind>> for Error {
    fn from(e: Box<bincode::ErrorKind>) -> Self {
        Error::IOCoreError(format!("{}", e))
    }
}
impl From<FromUtf8Error> for Error {
    fn from(e: FromUtf8Error) -> Self {
        Error::InvalidUtf8(format!("{}", e))
    }
}
impl From<FromHexError> for Error {
    fn from(e: FromHexError) -> Self {
        Error::DecodingError(format!("{}", e))
    }
}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(format!("{}", e))
    }
}
pub type Result<T> = std::result::Result<T, Error>;

pub fn ok() -> Result<()> {
    Ok(())
}
