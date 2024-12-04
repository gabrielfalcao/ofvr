use crate::models::author::Author;
use crate::Result;
use iocore::Path;
use pqpfs::{
    from_deflate_bytes, to_flate_bytes, DecryptionKey, EncryptionKey, PlainBytes, RSAPrivateKey,
};
use serde::{Deserialize, Serialize};

const CONF_HOME: &'static str = "~/.ofvr";

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct Conf {
    author: Author,
}
impl Conf {
    pub fn new(author: Author) -> Conf {
        Conf { author }
    }
    pub fn author(&self) -> Author {
        self.author.clone()
    }
    pub fn key_path(conf_path: &Path) -> Path {
        let key_path = conf_path.with_extension(".ky");
        key_path.with_filename(&format!(".{}", &key_path.name()))
    }
    pub fn load_from_file(path: impl Into<Path>) -> Result<Conf> {
        let path = path.into();
        let key_path = Conf::key_path(&path);
        let private_key = RSAPrivateKey::from_deflate_bytes(&key_path.read_bytes()?)?;
        let ciphertext = path.read_bytes()?;
        let plaintext = private_key.decrypt(ciphertext.iter().map(|byte| *byte))?;
        Ok(bincode::deserialize::<Conf>(&plaintext.to_bytes())?)
    }
    pub fn load() -> Result<Conf> {
        Conf::load_from_file(CONF_HOME)
    }
    pub fn save_to_file(&self, path: impl Into<Path>) -> Result<()> {
        let path = path.into();
        let key_path = Conf::key_path(&path);
        let private_key = RSAPrivateKey::generate()?;
        let public_key = private_key.public_key();
        let bytes = bincode::serialize::<Conf>(self)?;
        let bytes = public_key.encrypt(bytes.iter().map(|byte| *byte))?;
        key_path.write(&private_key.to_flate_bytes()?)?;
        path.write(&bytes.to_bytes())?;
        Ok(())
    }
    pub fn save(&self) -> Result<()> {
        Ok(self.save_to_file(CONF_HOME)?)
    }
    pub fn to_flate_bytes(&self) -> Result<Vec<u8>> {
        Ok(to_flate_bytes(self)?)
    }
    pub fn from_deflate_bytes(bytes: &[u8]) -> Result<Conf> {
        Ok(from_deflate_bytes::<Conf>(bytes)?)
    }
}
