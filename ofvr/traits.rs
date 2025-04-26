use iocore::Path;
use serde::{Deserialize, Serialize};
use sha3::Digest;
pub use sha3::Sha3_384;

use crate::Result;

pub trait PlainBytes: for<'a> Deserialize<'a> + Serialize + Sized {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_plain_bytes()
    }
    fn from_bytes(bytes: &[u8]) -> Self {
        Self::from_plain_bytes(bytes)
            .expect(&format!("{}::from_plain_bytes", std::any::type_name::<Self>()))
    }
    fn to_plain_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).expect("bytes")
    }
    fn from_plain_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(bincode::deserialize::<Self>(&bytes).unwrap())
    }
    fn to_flate_bytes(&self) -> Result<Vec<u8>> {
        Ok(crate::to_flate_bytes(self).unwrap())
    }
    fn from_deflate_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(crate::from_deflate_bytes::<Self>(bytes).unwrap())
    }
    fn to_hex(&self) -> String {
        hex::encode(&self.to_bytes())
    }
    fn sha3384(&self) -> Vec<u8> {
        let mut sha3_384 = Sha3_384::new();
        sha3_384.update(&self.to_plain_bytes());
        sha3_384.finalize().to_vec()
    }
    fn id3384(&self) -> String {
        let bytes = self.sha3384();
        hex::encode(&bytes[..8])
    }

}

pub trait FileSystemBytes: PlainBytes {
    fn save_to_file(&self, path: impl Into<Path>) -> Result<()> {
        let path = path.into();
        path.write(&self.to_bytes())?;
        Ok(())
    }
    fn load_from_file(path: impl Into<Path>) -> Result<Self> {
        let path = path.into();
        Ok(Self::from_bytes(&path.read_bytes()?))
    }
}
