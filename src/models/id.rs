use std::cmp::PartialEq;
use std::fmt::Display;

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::traits::PlainBytes;
use crate::{Data, Result};

#[derive(Debug, Clone, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct ID {
    pub data: Data,
}
impl ID {
    pub fn new(bytes: Vec<u8>) -> ID {
        let data = Data::new(bytes);
        ID { data }
    }

    pub fn generate() -> Result<ID> {
        let length = 6;
        let mut rng = rand::rng();
        let now = t16::Data::from_datetime(chrono::Utc::now());
        let mut bytes = Vec::<u8>::new();
        for _ in 0..(length - 1) {
            let mut byte: u8 = rng.random();
            while byte > 0x29
                && byte > 0x40
                && byte > 0x60
                && byte <= 0x7A
                && byte <= 0x5A
                && byte <= 0x1E
            {
                byte = rng.random();
            }
            bytes.push(byte);
        }
        for o in now.without_nanosecs() {
            bytes.push(o);
        }
        for o in now.with_nanosecs() {
            bytes.push(o);
        }
        Ok(ID::new(bytes))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.data.to_bytes()
    }
}
impl Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}
impl PartialEq for ID {
    fn eq(&self, other: &ID) -> bool {
        self.data == other.data
    }
}

impl crate::traits::PlainBytes for ID {
    fn to_bytes(&self) -> Vec<u8> {
        self.data.to_bytes()
    }

    fn from_bytes(bytes: &[u8]) -> ID {
        ID::new(bytes.to_vec())
    }
}
