use crate::errors::{Error, Result};
use crate::io::read_data;
use crate::models::author::Author;
use crate::models::commit::Commit;
use crate::traits::FileSystemBytes;
use gdiff::AxisBoundary;
use gdiff::Diff;
use iocore::Path;
use pqpfs::RSAPublicKey;
use pqpfs::ID;
use pqpfs::{PlainBytes, RSAPrivateKey};
use serde::{Deserialize, Serialize};
pub use sha3::{Digest, Keccak256, Keccak256Full};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct OFVRState {
    commits: Vec<Commit>,
    path: Path,
    private_key: RSAPrivateKey,
    authors: BTreeMap<u16, Author>,
}

impl OFVRState {
    pub fn get_author(&self, author: u16) -> Result<Author> {
        if self.authors.is_empty() {
            return Err(Error::StateError(format!("no authors in state")));
        }
        match self.authors.get(&author) {
            Some(author) => Ok(author.clone()),
            None => Err(Error::StateError(format!(
                "author {} NOT present in state",
                author
            ))),
        }
    }
    pub fn get_author_id(&self, author: &Author) -> Result<u16> {
        for (id, val) in self.authors.iter() {
            if *author == *val {
                return Ok(*id);
            }
        }
        Err(Error::StateError(format!(
            "author id not found for {}",
            author
        )))
    }
    pub fn add_author(&mut self, author: &Author) -> Result<u16> {
        let id = author.id();
        match self.get_author_id(author) {
            Ok(_) => Err(Error::StateError(format!(
                "author already stored: {}",
                author
            ))),
            Err(_) => {
                self.authors.insert(id, author.clone());
                Ok(id)
            }
        }
    }
    pub fn remove_author(&mut self, author_id: u16) -> Result<Author> {
        match self.authors.remove(&author_id) {
            Some(author) => Ok(author),
            None => Err(Error::StateError(format!(
                "author id {} not found",
                author_id
            ))),
        }
    }
    pub fn empty(path: &Path, author: &Author) -> Result<OFVRState> {
        let mut authors = BTreeMap::<u16, Author>::new();
        let commits = Vec::new();
        authors.insert(author.id(), author.clone());
        let path = path.clone();
        Ok(OFVRState {
            commits: commits.into(),
            authors,
            path,
            private_key: RSAPrivateKey::generate()?,
        })
    }
    pub fn store(&self) -> Result<()> {
        self.path.write(&self.to_plain_bytes())?;
        Ok(())
    }
    pub fn path(&self) -> Path {
        self.path.clone()
    }
    pub fn from_path(path: &Path) -> Result<OFVRState> {
        let data = read_data(path)?;
        Ok(OFVRState::from_plain_bytes(&data)?)
    }
    pub fn get_decryption_key_for_commit(&self, id: &ID) -> Result<Option<RSAPrivateKey>> {
        for commit in &self.commits {
            if commit.id() == *id {
                return Ok(Some(commit.data(self)?.decryption_key()));
            }
        }
        Ok(None)
    }
    pub fn get_decryption_key_for_new_commit(&self, author: u16) -> Result<RSAPrivateKey> {
        Ok(match self.latest_commit() {
            Some(commit) => commit.data(self)?.decryption_key(),
            None => self.get_author(author)?.private_key().clone(),
        })
    }
    pub fn get_encryption_key_for_new_commit(&self, author: u16) -> Result<RSAPublicKey> {
        Ok(match self.latest_commit() {
            Some(commit) => commit.public_key(),
            None => self.get_author(author)?.public_key(),
        })
    }
    pub fn latest_commit(&self) -> Option<Commit> {
        match self.commits.last() {
            Some(commit) => Some(commit.clone()),
            None => None,
        }
    }
    pub fn first_commit(&self) -> Option<Commit> {
        match self.commits.first() {
            Some(commit) => Some(commit.clone()),
            None => None,
        }
    }
    pub fn commits(&self) -> &[Commit] {
        &self.commits
    }
    pub fn commit(&mut self, data_path: &Path, author: &Author, message: &str) -> Result<()> {
        let data = read_data(&data_path)?;
        self.commit_blob(&data, author, message)
    }
    pub fn commit_blob(&mut self, data: &[u8], author: &Author, message: &str) -> Result<()> {
        let mut diff = match self.latest_commit() {
            Some(commit) => commit.data(&self)?.diff(),
            None => Diff::new(AxisBoundary::default()),
        };
        diff.update(data)?;
        let commit = Commit::now(diff, self.get_author_id(author)?, message, &self.path, self)?;
        self.commits.push(commit);
        self.store()?;
        Ok(())
    }
}
impl PlainBytes for OFVRState {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_plain_bytes()
    }
    fn from_bytes(bytes: &[u8]) -> Self {
        Self::from_plain_bytes(bytes).expect("OFVRState::from_plain_bytes")
    }
}
impl FileSystemBytes for OFVRState {
    fn default_path() -> Path {
        Path::cwd().join(".ofvr").try_canonicalize()
    }
}
