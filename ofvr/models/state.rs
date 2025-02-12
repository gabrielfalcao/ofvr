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
use crate::trace_info;
// use crate::models::commit_data::CommitData;
use crate::traits::FileSystemBytes;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct OFVRState {
    commits: Vec<Commit>,
    path: Path,
    private_key: String,
    authors: BTreeMap<u16, Author>,
}

impl OFVRState {
    pub fn private_key(&self) -> Result<RSAPrivateKey> {
        let bytes = hex::decode(&self.private_key)?;
        Ok(RSAPrivateKey::from_deflate_bytes(&bytes)?)
    }

    pub fn public_key(&self) -> Result<RSAPublicKey> {
        Ok(self.private_key()?.public_key())
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
        //trace_info!("OFVRState::", "empty");

        let mut authors = BTreeMap::<u16, Author>::new();
        let commits = Vec::new();
        authors.insert(author.id(), author.clone());
        let path = path.clone();
        let private_key = RSAPrivateKey::generate()?;
        trace_info!(
            "State::", "empty() -> \x1b[1;38;5;33mdecryption_key: {}\x1b[0m",
            private_key.id3384()
        );

        Ok(OFVRState {
            commits: commits.into(),
            authors,
            path,
            private_key: hex::encode(private_key.to_flate_bytes()?),
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
        //trace_info!("OFVRState::", "from_path()");
        Ok(OFVRState::load_from_file(path)?)
    }

    pub fn commits(&self) -> &[Commit] {
        //trace_info!("OFVRState::", "commits()");
        &self.commits
    }

    pub fn add_commit(&mut self, commit: Commit) -> Result<Commit> {
        //trace_info!("OFVRState::", "add_commit({})", &commit.id3384());
        self.commits.push(commit.clone());
        self.store()?;
        Ok(commit)
    }

    pub fn latest_commit(&self) -> Option<Commit> {
        match self.commits.last() {
            Some(commit) => {
                trace_info!("OFVRState::", "latest_commit() -> {}", &commit.id3384());
                Some(commit.clone())
            },
            None => {
                trace_info!("OFVRState::", "latest_commit() -> None");
                None
            },
        }
    }

    pub fn first_commit(&self) -> Option<Commit> {
        trace_info!("OFVRState::", "first_commit");
        match self.commits.first() {
            Some(commit) => Some(commit.clone()),
            None => None,
        }
    }

    pub fn commit_blob(&mut self, data: &[u8], author: &Author, message: &str) -> Result<Commit> {
        // trace_info!("OFVRState::", "commit_blob");
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
        let encryption_key = self.get_encryption_key_for_new_commit(author_id)?;
        diff.update(data)?;
        Ok(self.add_commit(Commit::now(
            diff,
            author_id,
            message,
            &self.path,
            self,
            &encryption_key,
        )?)?)
    }
}

impl OFVRState {
    pub fn commit(&mut self, data_path: &Path, author: &Author, message: &str) -> Result<Commit> {
        // trace_info!("OFVRState::", "commit");
        let data = read_data(&data_path)?;
        Ok(self.commit_blob(&data, author, message)?)
    }

    pub fn get_decryption_key_for_commit_no(&self, pos: usize) -> Result<RSAPrivateKey> {
        if pos == 0 {
            let private_key = self.private_key()?;
            trace_info!("OFVRState::", "get_decryption_key_for_commit_no {} \x1b[1;38;5;118mdecryption_key={}\x1b[0m", pos, private_key.id3384());
            return Ok(private_key);
        }

        let commits = self.commits();
        let len = commits.len();
        let commit = &commits[pos];
        let data = Commit::decrypt_commit_data(
            &self.get_decryption_key_for_commit_no(pos - 1)?,
            &commit.encrypted_data(),
        )
        .map_err(|error| {
            Error::StateError(format!(
                "failed to get decryption key for commit {}/{}: {}",
                pos + 1,
                len,
                error
            ))
        })?;
        Ok(data.decryption_key.clone())
    }

    pub fn get_decryption_key_for_commit(&self, id: &ID) -> Result<Option<RSAPrivateKey>> {
        // trace_info!("OFVRState::", "get_decryption_key_for_commit {}", id);
        let len = self.commits.len();
        if len == 0 {
            return Err(Error::StateError(String::from("no commits present in the current state")));
        }
        match self
            .commits()
            .iter()
            .enumerate()
            .find(|(_, &ref commit)| commit.id == *id)
            .map(|(pos, _)| self.get_decryption_key_for_commit_no(pos))
        {
            Some(data) => Ok(Some(data?)),
            None => return Err(Error::StateError(format!("no commit matching id {}", id))),
        }
    }

    pub fn get_encryption_key_for_new_commit(&self, author: u16) -> Result<RSAPublicKey> {
        // trace_info!("OFVRState::", "get_encryption_key_for_new_commit");
        Ok(match self.latest_commit() {
            Some(commit) => commit.public_key(),
            None => self.get_author(author)?.public_key()?,
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
