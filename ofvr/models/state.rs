use std::collections::BTreeMap;

use gdiff::{AxisBoundary, Diff};
use iocore::Path;
use pqpfs::{PlainBytes, RSAPrivateKey, RSAPublicKey, ID};
use serde::{Deserialize, Serialize};
pub use sha3::{Digest, Keccak256, Keccak256Full};

use crate::errors::{Error, Result};
use crate::io::read_data;
use crate::models::author::Author;
use crate::models::commit::Commit;
// use crate::models::commit_data::CommitData;
use crate::traits::FileSystemBytes;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct OFVRState {
    commits: Vec<Commit>,
    path: Path,
    private_key: RSAPrivateKey,
    authors: BTreeMap<u16, Author>,
}

impl OFVRState {
    pub fn public_key(&self) -> RSAPublicKey {
        self.private_key.public_key()
    }
    pub fn get_author(&self, author: u16) -> Result<Author> {
        if self.authors.is_empty() {
            return Err(Error::StateError(format!("no authors in state")));
        }
        match self.authors.get(&author) {
            Some(author) => Ok(author.clone()),
            None => Err(Error::StateError(format!("author {} NOT present in state", author))),
        }
    }

    pub fn get_author_id(&self, author: &Author) -> Result<u16> {
        for (id, val) in self.authors.iter() {
            if *author == *val {
                return Ok(*id);
            }
        }
        Err(Error::StateError(format!("author id not found for {}", author)))
    }

    pub fn add_author(&mut self, author: &Author) -> Result<u16> {
        let id = author.id();
        match self.get_author_id(author) {
            Ok(_) => Err(Error::StateError(format!("author already stored: {}", author))),
            Err(_) => {
                self.authors.insert(id, author.clone());
                Ok(id)
            },
        }
    }

    pub fn remove_author(&mut self, author_id: u16) -> Result<Author> {
        match self.authors.remove(&author_id) {
            Some(author) => Ok(author),
            None => Err(Error::StateError(format!("author id {} not found", author_id))),
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

    pub fn commits(&self) -> &[Commit] {
        &self.commits
    }

    pub fn add_commit(&mut self, commit: Commit) -> Result<Commit> {
        self.commits.push(commit.clone());
        self.store()?;
        Ok(commit)
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

    pub fn commit_blob(&mut self, data: &[u8], author: &Author, message: &str) -> Result<Commit> {
        let mut diff = match self.latest_commit() {
            Some(commit) => commit.data(&self)?.diff(),
            None => Diff::new(AxisBoundary::default()),
        };
        diff.update(data)?;
        Ok(self.add_commit(Commit::now(
            diff,
            self.get_author_id(author)?,
            message,
            &self.path,
            self,
        )?)?)
    }
}

impl OFVRState {
    pub fn commit(&mut self, data_path: &Path, author: &Author, message: &str) -> Result<Commit> {
        let data = read_data(&data_path)?;
        self.commit_blob(&data, author, message)
    }

    pub fn get_decryption_key_for_commit(&self, id: &ID) -> Result<Option<RSAPrivateKey>> {
        let pos = 0;
        let len = self.commits.len();
        if len == 0 {
            return Err(Error::StateError(String::from("no commits present in the current state")));
        }
        let mut decryption_key = self.private_key.clone();
        let mut commit = &self.commits[pos];
        let mut data = commit.encrypted_data();
        let commit_data = Commit::decrypt_commit_data(&decryption_key, data);
        if &commit.id == id {
            return Ok(Some(decryption_key));
        } else if len < 2 {
            return Err(Error::StateError(format!("no commits matching id {}", id)));
        }
        if let Err(error) = commit_data.clone() {
            return Err(Error::StateError(format!(
                "checking commit {}/{} failed to decrypt commit id {}: {}",
                pos, len, id, error
            )));
        }
        let mut commit_data = commit_data.unwrap();
        decryption_key = commit_data.decryption_key.clone();
        for pos in 1..len {
            commit = &self.commits[pos];
            data = commit.encrypted_data();
            match Commit::decrypt_commit_data(&decryption_key, data) {
                Err(error) =>
                    return Err(Error::StateError(format!(
                        "checking commit {}/{} failed to decrypt commit id {}: {}",
                        pos, len, id, error
                    ))),
                Ok(some_commit_data) => {
                    commit_data = some_commit_data;
                    decryption_key = commit_data.decryption_key.clone();
                    if &commit.id == id {
                        return Ok(Some(decryption_key));
                    }
                },
            }
        }
        Ok(None)
    }

    pub fn get_encryption_key_for_new_commit(&self, author: u16) -> Result<RSAPublicKey> {
        Ok(match self.latest_commit() {
            Some(commit) => commit.public_key(),
            None => self.get_author(author)?.public_key(),
        })
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
