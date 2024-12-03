pub use sha3::{Digest, Keccak256, Keccak256Full};

pub fn keccak256(bytes: &[u8]) -> Vec<u8> {
    let mut keccak256 = Keccak256::new();
    keccak256.update(bytes);
    let keccak256 = keccak256.finalize();
    keccak256.to_vec()
}

pub fn keccak256_full(bytes: &[u8]) -> Vec<u8> {
    let mut keccak256 = Keccak256Full::new();
    keccak256.update(bytes);
    let keccak256 = keccak256.finalize();
    keccak256.to_vec()
}
