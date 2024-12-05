use std::fmt::Display;
use std::num::TryFromIntError;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    CommitError(String),
    DiffError(String),
    HexDecodeError(String),
    DecodeError(String),
    EncodeError(String),
    IOError(String),
    PQPFSError(String),
    BincodeError(String),
    TomlError(String),
    StateError(String),
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
            "{}{}",
            self.variant(),
            match self {
                Self::CommitError(s) => format!("{}", s),
                Self::DiffError(s) => format!("{}", s),
                Self::HexDecodeError(s) => format!("{}", s),
                Self::DecodeError(s) => format!("{}", s),
                Self::EncodeError(s) => format!("{}", s),
                Self::IOError(s) => format!("{}", s),
                Self::PQPFSError(s) => format!("{}", s),
                Self::BincodeError(s) => format!("{}", s),
                Self::TomlError(s) => format!("{}", s),
                Self::StateError(s) => format!("{}", s),
            }
        )
    }
}
impl Error {
    pub fn variant(&self) -> String {
        match self {
            Error::CommitError(_) => "CommitError",
            Error::DiffError(_) => "DiffError",
            Error::HexDecodeError(_) => "HexDecodeError",
            Error::DecodeError(_) => "DecodeError",
            Error::EncodeError(_) => "EncodeError",
            Error::IOError(_) => "IOError",
            Error::BincodeError(_) => "BincodeError",
            Error::TomlError(_) => "TomlError",
            Error::StateError(_) => "StateError",
            Error::PQPFSError(_) => "PQPFSError",
        }
        .to_string()
    }
}
impl From<pqpfs::Error> for Error {
    fn from(e: pqpfs::Error) -> Self {
        Error::PQPFSError(format!("{}", e))
    }
}
impl From<hex::FromHexError> for Error {
    fn from(e: hex::FromHexError) -> Self {
        Error::HexDecodeError(format!("{}", e))
    }
}
impl From<gdiff::Error> for Error {
    fn from(e: gdiff::Error) -> Self {
        Error::DiffError(format!("{}", e))
    }
}
impl From<iocore::Exception> for Error {
    fn from(e: iocore::Exception) -> Self {
        Error::IOError(format!("{}", e))
    }
}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(format!("{:#?}", e))
    }
}
impl From<Box<bincode::ErrorKind>> for Error {
    fn from(e: Box<bincode::ErrorKind>) -> Self {
        Error::BincodeError(format!("{:#?}", e))
    }
}
impl From<toml::ser::Error> for Error {
    fn from(e: toml::ser::Error) -> Self {
        Error::TomlError(format!("{}", e))
    }
}
impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Error::TomlError(format!("{}", e))
    }
}
impl From<TryFromIntError> for Error {
    fn from(e: TryFromIntError) -> Self {
        Error::DecodeError(format!("{}", e))
    }
}
impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
