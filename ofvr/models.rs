use serde::{Deserialize, Serialize};

use pqpfs::Data;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct File {
    data: Data,
}
impl File {
    pub fn new(data: impl Into<Data>) -> File {
        File { data: data.into() }
    }
}
