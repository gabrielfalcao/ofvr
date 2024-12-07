use crate::Result;
use iocore::Path;
use pqpfs::{DecryptionKey, EncryptionKey, PlainBytes, RSAPrivateKey};

pub trait FileSystemBytes: PlainBytes {
    fn default_path() -> Path;
    fn key_path(path: &Path) -> Path {
        let key_path = path.with_extension(".ky");
        key_path.with_filename(&format!(".{}", &key_path.name()))
    }
    fn save_new_key_for_path(path: &Path) -> Result<RSAPrivateKey> {
        let key_path = Self::key_path(path);
        let private_key = RSAPrivateKey::generate()?;
        key_path.write(&private_key.to_bytes())?;
        Ok(private_key)
    }
    fn load_key_for_path(path: &Path) -> Result<RSAPrivateKey> {
        let key_path = Self::key_path(path);
        let key = RSAPrivateKey::from_bytes(&key_path.read_bytes()?);
        Ok(key)
    }
    fn save_to_file(&self, path: impl Into<Path>) -> Result<()> {
        let path = path.into();
        let key = Self::save_new_key_for_path(&path)?;
        let bytes = key.encrypt(self.to_bytes().iter().map(|byte| *byte))?;
        path.write(&bytes.to_bytes())?;
        Ok(())
    }
    fn save(&self) -> Result<()> {
        Ok(self.save_to_file(Self::default_path())?)
    }
    fn load_from_file(path: impl Into<Path>) -> Result<Self> {
        let path = path.into();
        let key = Self::load_key_for_path(&path)?;
        let ciphertext = path.read_bytes()?;
        let plaintext = key.decrypt(ciphertext.iter().map(|byte| *byte))?;
        Ok(Self::from_bytes(&plaintext.to_bytes()))
    }
    fn load() -> Result<Self> {
        Self::load_from_file(Self::default_path())
    }
}
