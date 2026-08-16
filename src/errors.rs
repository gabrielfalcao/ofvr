use std::num::TryFromIntError;

use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("failed to commit {0}")]
    CommitError(String),
    #[error("failed to generate diff {0}")]
    DiffError(#[from] bt_diff::Error),
    #[error("failed to decode hexadecimal string {0}")]
    HexDecodeError(#[from] hex::FromHexError),
    #[error("failed decode data {0}")]
    DecodeError(String),
    #[error("failed encode data {0}")]
    EncodeError(String),
    #[error("IOCore error {0}")]
    IOCoreError(#[from] iocore::Error),
    #[error("I/O error {0}")]
    IOError(#[from] std::io::Error),

    #[error("Error transcoding binary data {0}")]
    PostcardError(#[from] postcard::Error),
    #[error("TOML serialization error {0}")]
    TomlSerializationError(#[from] toml::ser::Error),
    #[error("TOML deserialization error {0}")]
    TomlDeserializationError(#[from] toml::de::Error),
    #[error("corrupted state error {0}")]
    StateError(String),

    #[error("Error trying to parse integer {0}")]
    IntParsingAttemptError(#[from] TryFromIntError),
}

pub type Result<T> = std::result::Result<T, Error>;
