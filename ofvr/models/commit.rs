use gdiff::Diff;
use iocore::Path;
use pqpfs::{Data, DecryptionKey, EncryptionKey, PlainBytes, RSAPrivateKey, RSAPublicKey, ID};
use serde::{Deserialize, Serialize};
pub use sha3::{Digest, Keccak256, Keccak256Full};

use crate::{
    models::{author::Author, commit_data::CommitData, state::OFVRState},
    Error, Result,
};

#[derive(Debug, Clone, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct Commit {
    pub id: ID,
    data: Vec<u8>,
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
        self.encryption_key.clone()
    }

    pub fn data(&self, ofvr: &OFVRState) -> Result<CommitData> {
        match ofvr.get_decryption_key_for_commit(&self.id)? {
            Some(decryption_key) => match Commit::decrypt_commit_data(&decryption_key, &Data::from(&self.data)) {
                Some(commit_data) => Ok(commit_data),
                None => Err(Error::StateError(format!("failed to decrypt commit data from commit {}", &self.id))),
            }
            None => Err(Error::StateError(format!("no commit matching {}", &self.id))),
        }
    }
    pub fn encrypted_data(&self) -> Data {
        Data::from(&self.data)
    }

    pub fn author(&self, ofvr: &OFVRState) -> Result<Author> {
        Ok(ofvr.get_author(self.data(ofvr)?.author_id())?)
    }

    pub fn encrypt_commit_data(
        encryption_key: &RSAPublicKey,
        commit_data: &CommitData,
    ) -> Result<Data> {
        Ok(encryption_key.encrypt(commit_data.to_plain_bytes().iter().map(|byte| *byte))?)
    }

    pub fn decrypt_commit_data(
        decryption_key: &RSAPrivateKey,
        data: &Data,
    ) -> Option<CommitData> {
        let decrypted_data = match decryption_key.decrypt(data.iter()) {
            Ok(decrypted_data) => decrypted_data,
            Err(_) => return None
        };
        match CommitData::from_plain_bytes(&decrypted_data.to_bytes()) {
            Ok(commit_data) => Some(commit_data),
            Err(_) => None
        }
    }

    pub fn new(commit_data: CommitData, ofvr: &OFVRState) -> Result<Commit> {
        let encryption_key = commit_data.decryption_key().public_key();
        let data = encryption_key
            .encrypt(commit_data.to_plain_bytes().iter().map(|byte| *byte))?
            .to_bytes();
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
