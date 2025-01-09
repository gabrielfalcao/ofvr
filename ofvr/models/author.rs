use pqpfs::{DataSeq, DecryptionKey, EncryptionKey, PlainBytes, RSAPrivateKey, RSAPublicKey};
use serde::{Deserialize, Serialize};

use crate::Result;

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

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn email(&self) -> String {
        self.email.to_string()
    }

    pub fn id(&self) -> u16 {
        let hash = crate::hash::keccak256(self.to_string().as_bytes());
        u16::from_ne_bytes([hash[1], hash[6]])
    }

    pub fn private_key(&self) -> Result<RSAPrivateKey> {
        let bytes = hex::decode(&self.private_key)?;
        Ok(RSAPrivateKey::from_deflate_bytes(&bytes)?)
    }

    pub fn public_key(&self) -> Result<RSAPublicKey> {
        Ok(self.private_key()?.public_key())
    }
}
impl EncryptionKey for Author {
    fn encrypt_bytes(&self, bytes: &[u8]) -> pqpfs::Result<DataSeq> {
        self.public_key()
            .map_err(|e| pqpfs::Error::HexDecodeError(e.to_string()))?
            .encrypt_bytes(bytes)
    }
}
impl DecryptionKey for Author {
    fn decrypt_bytes(&self, data: DataSeq) -> pqpfs::Result<DataSeq> {
        self.private_key()
            .map_err(|e| pqpfs::Error::HexDecodeError(e.to_string()))?
            .decrypt_bytes(data)
    }
}
impl PlainBytes for Author {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_plain_bytes()
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        Self::from_plain_bytes(bytes).expect("Author::from_plain_bytes")
    }
}
