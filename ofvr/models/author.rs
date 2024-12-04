use crate::Result;
use pqpfs::{
    DecryptionKey, EncryptionKey, PlainBytes, RSAPrivateKey,
    RSAPublicKey,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct Author {
    name: String,
    email: String,
    private_key: RSAPrivateKey,
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
            private_key: RSAPrivateKey::generate()?
        })
    }
    pub fn name(&self) -> String {
        self.name.to_string()
    }
    pub fn email(&self) -> String {
        self.email.to_string()
    }
    // pub fn from_conf(conf: &Conf) -> Author {
    //     conf.author()
    // }
    pub fn private_key(&self) -> &RSAPrivateKey {
        &self.private_key
    }
    pub fn public_key(&self) -> RSAPublicKey {
        self.private_key.public_key()
    }
    pub fn encrypt(&self, bytes: &[u8]) -> Result<Vec<u8>> {
        Ok(self
            .public_key()
            .encrypt(bytes.iter().map(|byte| byte.clone()))?
            .to_bytes())
    }
    pub fn decrypt(&self, bytes: &[u8]) -> Result<Vec<u8>> {
        Ok(self
            .private_key()
            .decrypt(bytes.iter().map(|byte| byte.clone()))?
            .to_bytes())
    }
}
