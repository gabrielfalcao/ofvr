use std::io::{Read, Write};

use flate2::read::DeflateDecoder;
use flate2::write::DeflateEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};

use crate::errors::Result;

pub fn xor_ip(a: &mut Vec<u8>, o: &Vec<u8>) {
    let alen = a.len();
    let olen = o.len();

    let max = if alen > olen {
        alen
    } else if olen > alen {
        olen
    } else {
        olen
    };

    for k in 0..max {
        if k < max {
            a[k] = a[k] ^ o[k];
        } else {
            break;
        }
    }
}
pub fn xor(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    a.into_iter().zip(b.iter()).map(|(a, b)| a ^ b).collect()
}

pub fn to_flate_bytes<T: Serialize>(data: &T) -> Result<Vec<u8>> {
    let bytes = bincode::serialize(data).unwrap();
    let mut e = DeflateEncoder::new(Vec::with_capacity(bytes.len()), Compression::best());
    e.write_all(&bytes)?;
    Ok(e.finish()?)
}

pub fn from_deflate_bytes<T: for<'a> Deserialize<'a>>(bytes: &[u8]) -> Result<T> {
    let mut d = DeflateDecoder::new(bytes);
    let mut bytes = Vec::<u8>::with_capacity(bytes.len());
    d.read_to_end(&mut bytes).unwrap();
    Ok(bincode::deserialize::<T>(&bytes).unwrap())
}

pub fn chunk_padded(items: &[u8], chunk_size: usize, padding: u8) -> Vec<Vec<u8>> {
    let rem = rem(items, chunk_size);
    let mut items = items.iter().map(|byte| *byte).collect::<Vec<u8>>();

    for _ in 0..rem {
        items.push(padding)
    }

    let mut chunked = Vec::<Vec<u8>>::new();
    for chunk in items.chunks(chunk_size) {
        chunked.push(chunk.to_vec());
    }
    chunked
}
pub(crate) fn rem(items: &[u8], chunk_size: usize) -> usize {
    if items.len() > chunk_size {
        items.len() % chunk_size
    } else if items.len() > 0 && chunk_size > items.len() {
        chunk_size % items.len()
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rem() {
        let data = vec![0x01, 0x10, 0xF1, 0x61];
        assert_eq!(rem(&data, 6), 2);
        let data = vec![0x01, 0x10, 0xF1, 0x61, 0x01, 0x10, 0xF1, 0x61];
        assert_eq!(rem(&data, 6), 2);
        let data = vec![];
        assert_eq!(rem(&data, 6), 0);
    }
}
