use crate::models::author::Author;
use crate::traits::FileSystemBytes;
use iocore::Path;
use pqpfs::PlainBytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct Conf {
    author: Author,
}
impl Conf {
    pub fn new(author: Author) -> Conf {
        Conf { author }
    }
    pub fn author(&self) -> Author {
        self.author.clone()
    }
}

impl PlainBytes for Conf {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_plain_bytes()
    }
    fn from_bytes(bytes: &[u8]) -> Self {
        Self::from_plain_bytes(bytes).expect("Conf::from_plain_bytes")
    }
}
impl FileSystemBytes for Conf {
    fn default_path() -> Path {
        Path::new("~/.ofvr").try_canonicalize()
    }
}
