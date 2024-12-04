use crate::io::read_data;
use crate::{Error, Result};
use flate2::write::{DeflateDecoder, DeflateEncoder};
use flate2::Compression;
use gdiff::AxisBoundary;
use gdiff::Diff;
use iocore::Path;
use pqpfs::{
    from_deflate_bytes, to_flate_bytes, DecryptionKey, EncryptionKey, PlainBytes, RSAPrivateKey,
    RSAPublicKey, ID,
};
use serde::{Deserialize, Serialize};
pub use sha3::{Digest, Keccak256, Keccak256Full};
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
        write!(f, "{} <{}>", &self.name, &self.email)
    }
}
impl Author {
    pub fn new(name: &str, email: &str) -> Result<Author> {
        Ok(Author {
            name: name.to_string(),
            email: email.to_string(),
            private_key: hex::encode(RSAPrivateKey::generate()?.to_flate_bytes()?),
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
pub struct CommitData {
    date: t16::Data,
    diff: Diff,
    message: String,
    path: Path,
    author: u16,
    decryption_key: RSAPrivateKey,
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
    pub fn now(diff: Diff, author: u16, message: &str, path: &Path) -> Result<CommitData> {
        let date = t16::Data::now();
        CommitData::new(&date, diff, author, message, path)
    }
    pub fn id(&self) -> Result<ID> {
        let mut keccak256 = Keccak256::new();
        keccak256.update(&to_flate_bytes(self)?);
        let keccak256 = keccak256.finalize();
        Ok(ID::new(keccak256.to_vec()))
    }

    pub fn to_bytes<K: EncryptionKey>(&self, encryption_key: K) -> Result<Vec<u8>> {
        let bytes = to_flate_bytes(self)?;
        Ok(encryption_key.encrypt(bytes.iter().map(|s| *s))?.into())
    }

    pub fn from_bytes<K: DecryptionKey>(bytes: &[u8], decryption_key: K) -> Result<CommitData> {
        let data = decryption_key.decrypt(bytes.iter().map(|s| *s))?;
        Ok(from_deflate_bytes::<CommitData>(&data.to_bytes())?)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct Commit {
    id: ID,
    data: Vec<u8>,
    author: RSAPublicKey,
    encryption_key: RSAPublicKey,
}

impl Commit {
    pub fn log(&self, ofvr: &OFVRState) -> Result<String> {
        let data = self.data(ofvr)?;
        Ok([
            format!("Commit: {}", self.id().to_hex("", false)),
            format!("Author: {}", &self.author(&ofvr)?),
            format!("Date: {}", data.date_rfc2822()),
            format!("\t{}\n", data.message()),
        ]
        .join("\n"))
    }
    pub fn id(&self) -> ID {
        self.id.clone()
    }
    pub fn public_key(&self) -> RSAPublicKey {
        self.encryption_key.clone()
    }
    pub fn data(&self, ofvr: &OFVRState) -> Result<CommitData> {
        let id = self.id();
        match ofvr.get_decryption_key_for_commit(&id)? {
            Some(decryption_key) => Ok(CommitData::from_bytes(&self.data, decryption_key)?),
            None => Err(Error::StateError(format!("no commit matching {}", &id))),
        }
    }
    pub fn author(&self, ofvr: &OFVRState) -> Result<Author> {
        Ok(ofvr.get_author(self.data(ofvr)?.author)?)
    }
    pub fn new(
        date: &t16::Data,
        diff: Diff,
        author: u16,
        message: &str,
        path: &Path,
        ofvr: &OFVRState,
    ) -> Result<Commit> {
        let commit_data = CommitData::new(date, diff, author, message, path)?;
        let encryption_key = commit_data.decryption_key().public_key();
        let data = commit_data.to_bytes(encryption_key.clone())?;
        let author = ofvr.get_author(author)?.public_key()?;
        let id = commit_data.id()?;
        Ok(Commit {
            id,
            data,
            author,
            encryption_key,
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
        Commit::new(&date, diff, author, message, path, ofvr)
    }

    pub fn to_flate_bytes(&self) -> Result<Vec<u8>> {
        Ok(to_flate_bytes(self)?)
    }

    pub fn from_deflate_bytes(bytes: &[u8]) -> Result<RSAPrivateKey> {
        Ok(from_deflate_bytes::<RSAPrivateKey>(bytes)?)
    }
}

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
                "author {} present in state",
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
            None => self.get_author(author)?.private_key()?,
        })
    }
    pub fn get_encryption_key_for_new_commit(&self, author: u16) -> Result<RSAPublicKey> {
        Ok(match self.latest_commit() {
            Some(commit) => commit.public_key(),
            None => self.get_author(author)?.public_key()?,
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
    pub fn empty(path: &Path, author: &Author) -> Result<OFVRState> {
        let mut authors = BTreeMap::<u16, Author>::new();
        let commits = Vec::new();
        let id: u16 = (authors.len() + 1).try_into().expect("u16");
        authors.insert(id, author.clone());
        let path = path.clone();
        Ok(OFVRState {
            commits: commits.into(),
            authors,
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
