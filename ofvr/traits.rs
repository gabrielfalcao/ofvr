use crate::Result;
use iocore::Path;
use pqpfs::{DecryptionKey, EncryptionKey, PlainBytes, RSAPrivateKey};

pub trait FileSystemBytes: PlainBytes {
    fn default_path() -> Path;
    fn key_path(path: &Path) -> Path {
        let key_path = path.with_extension(".ky");
        key_path.with_filename(&format!(".{}", &key_path.name()))
    }
    fn load_from_file(path: impl Into<Path>) -> Result<Self> {
        let path = path.into();
        // let key_path = Self::key_path(&path);
        // let private_key = RSAPrivateKey::from_deflate_bytes(&key_path.read_bytes()?)?;
        // let ciphertext = path.read_bytes()?;
        // let plaintext = private_key.decrypt(ciphertext.iter().map(|byte| *byte))?;
        // Ok(Self::from_deflate_bytes(&plaintext.to_plain_bytes())?)
        Ok(Self::from_bytes(&path.read_bytes()?))
    }
    fn load() -> Result<Self> {
        Self::load_from_file(Self::default_path())
    }
    fn save_to_file(&self, path: impl Into<Path>) -> Result<()> {
        let path = path.into();
        // let key_path = Self::key_path(&path);
        // let private_key = RSAPrivateKey::generate()?;
        // let public_key = private_key.public_key();
        // let bytes = self.to_flate_bytes()?;
        // let bytes = public_key.encrypt(bytes.iter().map(|byte| *byte))?;
        // key_path.write(&private_key.to_flate_bytes()?)?;
        // path.write(&bytes.to_plain_bytes())?;
        path.write(&self.to_bytes())?;
        Ok(())
    }
    fn save(&self) -> Result<()> {
        Ok(self.save_to_file(Self::default_path())?)
    }
}
