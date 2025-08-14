use std::iter::{Extend, IntoIterator, Iterator};
use std::ops::Index;

use serde::{Deserialize, Serialize};

use super::core::Data;
use crate::traits::PlainBytes;
use crate::Result;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct DataSeq {
    seq: Vec<Data>,
    length: usize,
}

impl DataSeq {
    pub fn new() -> DataSeq {
        DataSeq {
            seq: Vec::new(),
            length: 0,
        }
    }

    pub fn to_vec(&self) -> Vec<Data> {
        self.seq.clone()
    }

    pub fn to_data(&self) -> Result<Data> {
        Ok(Data::from(self.to_plain_bytes()))
    }

    pub fn from_data(data: &Data) -> Result<DataSeq> {
        DataSeq::from_plain_bytes(&data.to_bytes())
    }

    pub fn iter(&self) -> DataSeqIterator {
        DataSeqIterator::new(self)
    }

    pub fn get(&mut self, index: usize) -> Option<Data> {
        self.seq.get(index).map(|byte| byte.clone())
    }

    pub fn push(&mut self, byte: Data) {
        self.length += 1;
        self.seq.push(byte);
    }

    pub fn pop(&mut self) -> Option<Data> {
        match self.seq.pop() {
            Some(data) => {
                self.length -= 1;
                Some(data)
            },
            None => None,
        }
    }

    pub fn len(&self) -> usize {
        self.seq.len()
    }

    pub fn is_empty(&self) -> bool {
        self.seq.is_empty()
    }

    pub fn extend<T: Iterator<Item = Data>>(&mut self, iter: T) {
        self.seq.extend(iter);
        self.length = self.seq.len();
    }

    pub fn extended<T: Iterator<Item = Data>>(&self, iter: T) -> DataSeq {
        let mut data = self.clone();
        data.extend(iter);
        data
    }
}
impl Index<usize> for DataSeq {
    type Output = Data;

    fn index(&self, index: usize) -> &Self::Output {
        &self.seq[index]
    }
}

impl Into<Data> for DataSeq {
    fn into(self) -> Data {
        self.to_data().expect("data bytes")
    }
}
impl From<&Data> for DataSeq {
    fn from(data: &Data) -> DataSeq {
        DataSeq::from_data(data).expect("data seq bytes")
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct DataSeqIterator {
    seq: DataSeq,
    pos: usize,
}

impl DataSeqIterator {
    pub fn new(seq: &DataSeq) -> DataSeqIterator {
        DataSeqIterator {
            seq: seq.clone(),
            pos: 0,
        }
    }
}

impl Iterator for DataSeqIterator {
    type Item = Data;

    fn next(&mut self) -> Option<Data> {
        let item = self.seq.get(self.pos);
        self.pos += 1;
        item
    }
}

impl IntoIterator for DataSeq {
    type IntoIter = DataSeqIterator;
    type Item = Data;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl PlainBytes for DataSeq {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_plain_bytes()
    }

    fn from_bytes(bytes: &[u8]) -> DataSeq {
        DataSeq::from_plain_bytes(bytes).expect("DataSeq::from_plain_bytes")
    }
}
