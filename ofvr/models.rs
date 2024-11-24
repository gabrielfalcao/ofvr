use crate::io::read_data;
use crate::{Error, Result};
use pqpfs::{RSAPrivateKey, RSAPublicKey};
use flate2::write::{DeflateDecoder, DeflateEncoder};
use flate2::Compression;
use gdiff::AxisBoundary;
use gdiff::Diff;
use iocore::Path;
use pqpfs::Data;
use serde::{Deserialize, Serialize};
use std::collections::vec_deque::VecDeque;
use std::collections::BTreeMap;
use std::io::Read;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct Commit {
    date: t16::Data,
    diff: Diff,
    message: String,
    path: Path,
    author: String,
    optional_metadata: BTreeMap<String, String>,
    // encryption_key: RSAPublicKey,
    // signing_key: RSAPrivateKey,
}

impl Commit {
    pub fn date(&self) -> t16::Data {
        self.date.clone()
    }
    pub fn date_rfc2822(&self) -> String {
        self.date().to_chrono().to_rfc2822()
    }
    pub fn diff(&self) -> Diff {
        self.diff.clone()
    }
    pub fn author(&self) -> String {
        self.author.clone()
    }
    pub fn message(&self) -> String {
        self.message.clone()
    }
    pub fn path(&self) -> Path {
        self.path.clone()
    }
    pub fn optional_metadata(&self) -> BTreeMap<String, String> {
        self.optional_metadata.clone()
    }
    pub fn new(date: &t16::Data, diff: Diff, author: &str, message: &str, path: &Path) -> Commit {
        let author = author.to_string();
        let date = date.clone();
        let message = message.to_string();
        let optional_metadata = BTreeMap::<String, String>::new();
        let path = path.clone();

        Commit {
            author,
            date,
            diff,
            message,
            optional_metadata,
            path,
        }
    }
    pub fn now(diff: Diff, author: &str, message: &str, path: &Path) -> Commit {
        let date = t16::Data::from_datetime(chrono::Utc::now());
        Commit::new(&date, diff, author, message, path)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct OFVRState {
    commits: VecDeque<Commit>,
    path: Path,
    private_key: RSAPrivateKey
}

impl OFVRState {
    pub fn new_with_commit_blob(
        path: &Path,
        author: &str,
        message: &str,
        data: Data,
    ) -> Result<OFVRState> {
        let mut state = OFVRState::empty(path)?;
        state.commit_blob(data, author, message)?;
        Ok(state)
    }
    pub fn new_with_commit(
        path: &Path,
        author: &str,
        message: &str,
        data_path: &Path,
    ) -> Result<OFVRState> {
        let mut state = OFVRState::empty(path)?;
        state.commit(data_path, author, message)?;
        Ok(state)
    }
    pub fn latest_commit(&self) -> Option<Commit> {
        match self.commits.front() {
            Some(commit) => Some(commit.clone()),
            None => None,
        }
    }
    pub fn first_commit(&self) -> Option<Commit> {
        match self.commits.back() {
            Some(commit) => Some(commit.clone()),
            None => None,
        }
    }
    pub fn commits(&self) -> VecDeque<Commit> {
        self.commits.clone()
    }
    pub fn commit(&mut self, data_path: &Path, author: &str, message: &str) -> Result<Commit> {
        let data = read_data(&data_path)?;
        self.commit_blob(data, author, message)
    }
    pub fn commit_blob(&mut self, data: Data, author: &str, message: &str) -> Result<Commit> {
        let mut diff = match self.latest_commit() {
            Some(commit) => commit.diff(),
            None => Diff::new(AxisBoundary::default()),
        };
        diff.update(&data.bytes())?;
        let commit = Commit::now(diff, author, message, &self.path);
        self.commits.push_front(commit.clone());
        self.store()?;
        Ok(commit)
    }
    pub fn empty(path: &Path) -> Result<OFVRState> {
        let commits = VecDeque::new();
        let path = path.clone();
        Ok(OFVRState {
            commits: commits.into(),
            path,
            private_key: RSAPrivateKey::generate()?
        })
    }
    pub fn store(&self) -> Result<()> {
        self.path.write(&self.to_bytes()?)?;
        Ok(())
    }
    pub fn path(&self) -> Path {
        self.path.clone()
    }
    pub fn from_path(path: &Path) -> Result<OFVRState> {
        let data = read_data(path)?;
        Ok(OFVRState::from_bytes(&data.bytes())?)
    }
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let bytes = bincode::serialize(self)
            .map_err(|e| Error::EncodeError(e.to_string()))?
            .bytes()
            .map(|byte| byte.unwrap_or_default())
            .collect::<Vec<u8>>();
        let mut e = DeflateEncoder::new(Vec::new(), Compression::best());
        e.write(&bytes)?;
        Ok(e.finish()?)
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<OFVRState> {
        let mut d = DeflateDecoder::new(Vec::new());
        d.write(bytes)?;
        let deflated = d.finish()?;
        Ok(bincode::deserialize::<OFVRState>(&deflated)
            .expect("deserialize OFVRState from deflated bytes"))
    }
}
