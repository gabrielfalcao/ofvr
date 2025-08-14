use crate::models::author::Author;
use crate::traits::{FileSystemBytes, PlainBytes};
use iocore::Path;
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
    pub fn default_path() -> Path {
        Path::raw("~/.ofvr").try_canonicalize()
    }

}
impl PlainBytes for Conf {}
impl FileSystemBytes for Conf {}
