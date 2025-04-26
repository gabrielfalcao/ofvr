use bt_diff::Diff;
use iocore::Path;
use serde::{Deserialize, Serialize};
pub use sha3::{Digest, Keccak256, Keccak256Full};
use crate::traits::PlainBytes;

use crate::models::author::Author;
use crate::models::commit_data::CommitData;
use crate::models::state::OFVRState;
use crate::models::id::ID;
use crate::{Result, Data};

#[derive(Debug, Clone, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct Commit {
    pub id: ID,
    data: Data,
    author: u16,
}
impl PartialEq for Commit {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Commit {
    pub fn log(&self, ofvr: &OFVRState) -> Result<String> {
        let data = self.data(ofvr)?;
        Ok([
            format!("Commit: {}", self.id.to_hex()),
            format!("Author: {}", &self.author(&ofvr)?),
            format!("Date: {}", data.date_rfc2822()),
            format!("\t{}\n", data.message()),
        ]
        .join("\n"))
    }

    pub fn data(&self, _: &OFVRState) -> Result<CommitData> {
        Ok(CommitData::from_plain_bytes(&self.data.to_bytes())?)
    }

    pub fn author(&self, ofvr: &OFVRState) -> Result<Author> {
        Ok(ofvr.get_author(self.data(ofvr)?.author_id())?)
    }
    pub fn new(
        commit_data: CommitData,
        ofvr: &OFVRState,
    ) -> Result<Commit> {
        let data = Data::from(commit_data.to_plain_bytes());
        let author = commit_data.author(ofvr)?.id();
        let id = commit_data.id()?;

        Ok(Commit {
            id,
            data,
            author,
        })
    }

    pub fn now(
        diff: Diff,
        author: u16,
        message: &str,
        path: &Path,
        ofvr: &OFVRState,
    ) -> Result<Commit> {
        let date = t16::Data::now();
        let commit_data = CommitData::new(&date, diff, author, message, path)?;
        Commit::new(commit_data, ofvr)
    }
}
impl PlainBytes for Commit{}
