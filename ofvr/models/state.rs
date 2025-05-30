use std::collections::BTreeMap;

use bt_diff::{AxisBoundary, Diff};
use iocore::Path;
use serde::{Deserialize, Serialize};
pub use sha3::{Digest, Keccak256, Keccak256Full};

use crate::errors::{Error, Result};
use crate::io::read_data;
use crate::models::author::Author;
use crate::models::commit::Commit;
// use crate::models::commit_data::CommitData;
use crate::traits::{FileSystemBytes, PlainBytes};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct OFVRState {
    commits: Vec<Commit>,
    path: Path,
    authors: BTreeMap<u16, Author>,
}

impl OFVRState {
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
        })
    }

    pub fn store(&self) -> Result<()> {
        self.save_to_file(&self.path)?;
        Ok(())
    }

    pub fn path(&self) -> Path {
        self.path.clone()
    }

    pub fn from_path(path: &Path) -> Result<OFVRState> {
        Ok(OFVRState::load_from_file(path)?)
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
            Some(commit) => {
                Some(commit.clone())
            },
            None => {
                None
            },
        }
    }

    pub fn first_commit(&self) -> Option<Commit> {
        match self.commits.first() {
            Some(commit) => Some(commit.clone()),
            None => None,
        }
    }

    pub fn commit_blob(&mut self, data: &[u8], author: &Author, message: &str) -> Result<Commit> {
        let author_id = if let Ok(author_id) = self.get_author_id(author) {
            author_id
        } else {
            let author_id = self.add_author(author)?;
            self.store()?;
            author_id
        };
        let mut diff = match self.latest_commit() {
            Some(commit) => commit.data(&self)?.diff(),
            None => Diff::new(AxisBoundary::default()),
        };
        diff.update(data)?;
        Ok(self.add_commit(Commit::now(
            diff,
            author_id,
            message,
            &self.path,
            self,
        )?)?)
    }
}

impl OFVRState {
    pub fn commit(&mut self, data_path: &Path, author: &Author, message: &str) -> Result<Commit> {
        let data = read_data(&data_path)?;
        Ok(self.commit_blob(&data, author, message)?)
    }
}

impl PlainBytes for OFVRState{}
impl FileSystemBytes for OFVRState {}
