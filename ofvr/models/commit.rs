use gdiff::Diff;
use iocore::Path;
use pqpfs::{Data, DecryptionKey, EncryptionKey, PlainBytes, RSAPrivateKey, RSAPublicKey, ID};
use serde::{Deserialize, Serialize};
pub use sha3::{Digest, Keccak256, Keccak256Full};
use tracing::info;

use crate::models::author::Author;
use crate::models::commit_data::CommitData;
use crate::models::state::OFVRState;
use crate::{Error, Result};

#[derive(Debug, Clone, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct Commit {
    pub id: ID,
    data: Data,
    author: u16,
    encryption_key: RSAPublicKey,
}
impl PartialEq for Commit {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Commit {
    pub fn log(&self, ofvr: &OFVRState) -> Result<String> {
        info!("Commit.log()");
        let data = self.data(ofvr)?;
        Ok([
            format!("Commit: {}", self.id.to_hex("", false)),
            format!("Author: {}", &self.author(&ofvr)?),
            format!("Date: {}", data.date_rfc2822()),
            format!("\t{}\n", data.message()),
        ]
        .join("\n"))
    }

    pub fn public_key(&self) -> RSAPublicKey {
        info!("Commit.public_key()");
        self.encryption_key.clone()
    }

    pub fn data(&self, ofvr: &OFVRState) -> Result<CommitData> {
        info!("Commit.data()");
        let decryption_key = self.get_decryption_key_from_state(ofvr)?;
        Ok(Commit::decrypt_commit_data(&decryption_key, &self.encrypted_data())?)
    }

    pub fn get_decryption_key_from_state(&self, ofvr: &OFVRState) -> Result<RSAPrivateKey> {
        info!("Commit[{}].get_decryption_key_from_state()", self.id);
        match ofvr
            .get_decryption_key_for_commit(&self.id)
            .expect(&format!("get decryption key for commit {}", &self.id))
        {
            Some(decryption_key) => Ok(decryption_key),
            None => Err(Error::StateError(format!("no commit matching {}", &self.id))),
        }
    }

    pub fn encrypted_data(&self) -> &Data {
        info!("Commit[{}].encrypted_data() -> {}", self.id, self.data.id3384());
        &self.data
    }

    pub fn author(&self, ofvr: &OFVRState) -> Result<Author> {
        info!("Commit.author()");
        Ok(ofvr.get_author(self.data(ofvr)?.author_id())?)
    }

    pub fn encrypt_commit_data(
        encryption_key: &RSAPublicKey,
        commit_data: &CommitData,
    ) -> Result<Data> {
        info!("Commit.encrypt_commit_data()");
        Ok(encryption_key.encrypt(commit_data.to_plain_bytes().iter().map(|byte| *byte))?)
    }

    pub fn decrypt_commit_data(decryption_key: &RSAPrivateKey, data: &Data) -> Result<CommitData> {
        info!("Commit::decrypt_commit_data(decryption_key: {})", decryption_key.id3384());
        let decrypted_data = decryption_key.decrypt(data.iter())?;
        Ok(CommitData::from_plain_bytes(&decrypted_data.to_bytes())?)
    }

    pub fn new(commit_data: CommitData, ofvr: &OFVRState) -> Result<Commit> {
        info!("Commit.new()");
        let encryption_key = match ofvr.latest_commit() {
            Some(commit) => commit.public_key(),
            None => ofvr.public_key()?,
        };
        let data = Commit::encrypt_commit_data(&encryption_key, &commit_data)?;
        let author = commit_data.author(ofvr)?.id();
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
        info!("Commit.now()");
        let date = t16::Data::now();
        let commit_data = CommitData::new(&date, diff, author, message, path)?;
        Commit::new(commit_data, ofvr)
    }
}

impl PlainBytes for Commit {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_plain_bytes()
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        Self::from_plain_bytes(bytes).expect("Commit::from_plain_bytes")
    }
}
