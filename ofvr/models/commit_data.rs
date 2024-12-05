use crate::models::author::Author;
use crate::Result;
use gdiff::Diff;
use iocore::Path;
use pqpfs::{
    from_deflate_bytes, to_flate_bytes, DecryptionKey, EncryptionKey, PlainBytes, RSAPrivateKey, ID,
};
use serde::{Deserialize, Serialize};
pub use sha3::{Digest, Keccak256, Keccak256Full};

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
