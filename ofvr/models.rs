use crate::io::read_data;
use crate::{Error, Result};
use flate2::write::{DeflateDecoder, DeflateEncoder};
use flate2::Compression;
use gdiff::AxisBoundary;
use gdiff::Diff;
use iocore::Path;
use pqpfs::{RSAPrivateKey, RSAPublicKey};
use serde::{Deserialize, Serialize};
use std::collections::vec_deque::VecDeque;
use std::collections::BTreeMap;
use std::io::Read;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct Author {
    name: String,
    email: String,
    private_key: String,
}
impl std::fmt::Display for Author {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} <{}>", &self.name, &self.email,)
    }
}
impl Author {
    pub fn new(name: &str, email: &str) -> Result<Author> {
        Ok(Author {
            name: name.to_string(),
            email: email.to_string(),
            private_key: hex::encode(RSAPrivateKey::generate()?.to_flate_bytes()?)
        })
    }
    pub fn from_conf(conf: &Conf) -> Author {
        conf.author()
    }
    pub fn private_key(&self) -> Result<RSAPrivateKey> {
        let bytes = hex::decode(&self.private_key)?;
        Ok(RSAPrivateKey::from_deflate_bytes(&bytes)?)
    }
    pub fn public_key(&self) -> Result<RSAPublicKey> {
        Ok(self.private_key()?.public_key())
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct Conf {
    author: Author,
}
impl Conf {
    pub fn author(&self) -> Author {
        self.author.clone()
    }
    pub fn load() -> Result<Conf> {
        let path = Path::new("~/.ofvr").canonicalize()?;
        Ok(toml::from_str::<Conf>(&path.read()?)?)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct Commit {
    date: t16::Data,
    diff: Diff,
    message: String,
    path: Path,
    author: Author,
    optional_metadata: BTreeMap<String, String>,
    encryption_key: RSAPublicKey,
    signing_key: RSAPrivateKey,
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
    pub fn author(&self) -> Author {
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
    pub fn new(
        date: &t16::Data,
        diff: Diff,
        author: &Author,
        message: &str,
        path: &Path,
        ancestor_key: &RSAPublicKey,
    ) -> Result<Commit> {
        let encryption_key = ancestor_key.clone();
        let author = author.clone();
        let date = date.clone();
        let message = message.to_string();
        let optional_metadata = BTreeMap::<String, String>::new();
        let path = path.clone();

        Ok(Commit {
            author,
            date,
            diff,
            message,
            optional_metadata,
            path,
            encryption_key,
            signing_key: RSAPrivateKey::generate()?,
        })
    }
    pub fn now(
        diff: Diff,
        author: &Author,
        message: &str,
        path: &Path,
        ancestor_key: &RSAPublicKey,
    ) -> Result<Commit> {
        let date = t16::Data::from_datetime(chrono::Utc::now());
        Commit::new(&date, diff, author, message, path, ancestor_key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct OFVRState {
    commits: VecDeque<Commit>,
    path: Path,
    author: Author,
    private_key: RSAPrivateKey,
}

impl OFVRState {
    pub fn new_with_commit_blob(
        path: &Path,
        author: &Author,
        message: &str,
        data: &[u8],
    ) -> Result<OFVRState> {
        let mut state = OFVRState::empty(path, author)?;
        state.commit_blob(data, author, message)?;
        Ok(state)
    }
    pub fn new_with_commit(
        path: &Path,
        author: &Author,
        message: &str,
        data_path: &Path,
    ) -> Result<OFVRState> {
        let mut state = OFVRState::empty(path, author)?;
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
    pub fn commit(&mut self, data_path: &Path, author: &Author, message: &str) -> Result<Commit> {
        let data = read_data(&data_path)?;
        self.commit_blob(&data, author, message)
    }
    pub fn commit_blob(&mut self, data: &[u8], author: &Author, message: &str) -> Result<Commit> {
        let mut diff = match self.latest_commit() {
            Some(commit) => commit.diff(),
            None => Diff::new(AxisBoundary::default()),
        };
        let ancestor_key = match self.latest_commit() {
            Some(commit) => commit.signing_key.public_key(),
            None => author.public_key()?,
        };
        diff.update(data)?;
        let commit = Commit::now(diff, author, message, &self.path, &ancestor_key)?;
        self.commits.push_front(commit.clone());
        self.store()?;
        Ok(commit)
    }
    pub fn empty(path: &Path, author: &Author) -> Result<OFVRState> {
        let commits = VecDeque::new();
        let author = author.clone();
        let path = path.clone();
        Ok(OFVRState {
            commits: commits.into(),
            author,
            path,
            private_key: RSAPrivateKey::generate()?,
        })
    }
    pub fn store(&self) -> Result<()> {
        self.path.write(&self.to_bytes()?.to_vec())?;
        Ok(())
    }
    pub fn path(&self) -> Path {
        self.path.clone()
    }
    pub fn from_path(path: &Path) -> Result<OFVRState> {
        let data = read_data(path)?;
        Ok(OFVRState::from_bytes(&data)?)
    }
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let bytes = bincode::serialize(self)
            .map_err(|e| Error::EncodeError(e.to_string()))?
            .bytes()
            .map(|byte| byte.unwrap_or_default())
            .collect::<Vec<u8>>();
        let mut e = DeflateEncoder::new(Vec::new(), Compression::best());
        e.write_all(&bytes.to_vec())?;
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
