use bt_diff::Diff;
use iocore::Path;
use serde::{Deserialize, Serialize};
pub use sha3::{Digest, Keccak256, Keccak256Full};
use crate::traits::PlainBytes;

use crate::models::author::Author;
use crate::models::state::OFVRState;
use crate::models::id::ID;
use crate::Result;

#[derive(Debug, Clone, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct CommitData {
    date: t16::Data,
    diff: Diff,
    message: String,
    path: Path,
    author: u16,
}
impl PartialEq for CommitData {
    fn eq(&self, other: &Self) -> bool {
        self.diff == other.diff
    }
}
impl CommitData {
    pub fn date(&self) -> t16::Data {
        self.date.clone()
    }

    pub fn diff(&self) -> Diff {
        self.diff.clone()
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }

    pub fn author(&self, ofvr: &OFVRState) -> Result<Author> {
        Ok(ofvr.get_author(self.author)?)
    }

    pub fn author_id(&self) -> u16 {
        self.author
    }

    pub fn date_rfc2822(&self) -> String {
        self.date().to_chrono().to_rfc2822()
    }

    pub fn path(&self) -> Path {
        self.path.clone()
    }

    pub fn new(
        date: &t16::Data,
        diff: Diff,
        author: u16,
        message: &str,
        path: &Path,
    ) -> Result<CommitData> {
        let date = date.clone();
        let message = message.to_string();
        let path = path.clone();
        let commit_data = CommitData {
            date,
            diff,
            message,
            path,
            author,
        };
        Ok(commit_data)
    }

    pub fn id(&self) -> Result<ID> {
        let id = ID::new(crate::hash::keccak256(&self.to_flate_bytes().unwrap()));
        Ok(id)
    }

    pub fn date_rfc3339(&self) -> String {
        self.date().to_chrono().to_rfc3339()
    }
}
impl PlainBytes for CommitData {}
