use crate::models::author::Author;
use crate::models::state::OFVRState;
use crate::{FileSystemBytes, Result};
use gdiff::Diff;
use iocore::Path;
use pqpfs::{PlainBytes, RSAPrivateKey, ID};
use serde::{Deserialize, Serialize};
pub use sha3::{Digest, Keccak256, Keccak256Full};

#[derive(Debug, Clone, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct CommitData {
    date: t16::Data,
    diff: Diff,
    message: String,
    path: Path,
    author: u16,
    pub decryption_key: RSAPrivateKey,
}
impl PartialEq for CommitData {
    fn eq(&self, other: &Self) -> bool {
        self.decryption_key == other.decryption_key
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
    pub fn decryption_key(&self) -> RSAPrivateKey {
        self.decryption_key.clone()
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
        let decryption_key = RSAPrivateKey::generate()?;

        Ok(CommitData {
            date,
            diff,
            message,
            path,
            author,
            decryption_key,
        })
    }
    pub fn id(&self) -> Result<ID> {
        Ok(ID::new(crate::hash::keccak256(&self.to_flate_bytes()?)))
    }
    pub fn date_rfc3339(&self) -> String {
        self.date().to_chrono().to_rfc3339()
    }
}
impl PlainBytes for CommitData {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_plain_bytes()
    }
    fn from_bytes(bytes: &[u8]) -> Self {
        Self::from_plain_bytes(bytes).expect("CommitData::from_plain_bytes")
    }
}
impl FileSystemBytes for CommitData {
    fn default_path() -> Path {
        Path::cwd().join("...")
    }
}
