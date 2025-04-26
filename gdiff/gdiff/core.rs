use std::io::Write;

use enum_to_string::EnumToString;
use flate2::write::{DeflateDecoder, DeflateEncoder};
use flate2::Compression;
use serde::{Deserialize, Serialize};

use crate::Result;

pub const DEFAULT_AXIS_BOUNDARY_LEN: usize = 36;

#[derive(EnumToString, Clone, Copy, Deserialize, Serialize)]
pub enum DiffType {
    Binary,
    Text,
}
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum LineTermination {
    CR,
    LF,
    CRLF,
}
impl Default for LineTermination {
    fn default() -> LineTermination {
        LineTermination::CRLF
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, PartialOrd, Ord, Eq)]
pub enum AxisBoundary {
    Len(usize),
    // LineTermination(LineTermination),
    // Byte(u8),
    // Pair([u8; 2]),
    // Sequence(Vec<u8>),
}
impl Default for AxisBoundary {
    fn default() -> AxisBoundary {
        AxisBoundary::Len(DEFAULT_AXIS_BOUNDARY_LEN)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct DiffSettings {
    pub axis_boundary: AxisBoundary,
}
impl Default for DiffSettings {
    fn default() -> DiffSettings {
        DiffSettings {
            axis_boundary: AxisBoundary::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct DiffUnit {
    pub x: usize,
    pub y: usize,
    pub anterior: Vec<u8>,
    pub current: Vec<u8>,
}
impl DiffUnit {
    pub fn new(anterior: Option<u8>, current: Option<u8>, x: usize, y: usize) -> DiffUnit {
        DiffUnit {
            x,
            y,
            anterior: anterior.map(|byte| vec![byte]).unwrap_or_default(),
            current: current.map(|byte| vec![byte]).unwrap_or_default(),
        }
    }

    pub fn anterior(&self) -> Option<u8> {
        self.anterior.get(0).copied()
    }

    pub fn current(&self) -> Option<u8> {
        self.current.get(0).copied()
    }
}

#[derive(Clone, Deserialize, Serialize, Hash, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct Diff {
    pub sequence: Vec<DiffUnit>,
    pub settings: DiffSettings,
}
impl Default for Diff {
    fn default() -> Diff {
        Diff {
            sequence: Vec::new(),
            settings: DiffSettings::default(),
        }
    }
}
impl Diff {
    pub fn new(axis_boundary: AxisBoundary) -> Diff {
        Diff {
            sequence: Vec::new(),
            settings: DiffSettings { axis_boundary },
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let bytes = bincode::serialize(self)?;
        Ok(bytes)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Diff> {
        let diff: Diff = bincode::deserialize(bytes)?;
        Ok(diff)
    }

    pub fn to_flate_bytes(&self) -> Result<Vec<u8>> {
        let mut e = DeflateEncoder::new(Vec::new(), Compression::best());
        e.write(&self.to_bytes()?)?;
        Ok(e.finish()?)
    }

    pub fn from_deflate_bytes(bytes: &[u8]) -> Result<Diff> {
        let mut d = DeflateDecoder::new(Vec::new());
        d.write(bytes)?;
        let deflated = d.finish()?;
        Ok(Diff::from_bytes(&deflated)?)
    }

    pub fn anterior_version(&self) -> Vec<u8> {
        self.sequence
            .iter()
            .filter(|unit| unit.anterior().is_some())
            .map(|unit| unit.anterior().unwrap())
            .collect()
    }

    pub fn current_version(&self) -> Vec<u8> {
        self.sequence
            .iter()
            .filter(|unit| unit.current().is_some())
            .map(|unit| unit.current().unwrap())
            .collect()
    }

    pub fn update(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        let anterior = self.anterior_version();
        let current = self.current_version();
        let diff = diff(&current, data, self.axis_boundary())?;
        self.sequence = diff.sequence.clone();
        Ok(anterior)
    }

    pub fn axis_boundary(&self) -> AxisBoundary {
        self.settings.axis_boundary.clone()
    }

    pub fn digest_anterior_version<A: digest::Digest>(&self) -> Vec<u8> {
        let mut sha3 = A::new();
        sha3.update(&self.anterior_version());
        sha3.finalize().to_vec()
    }

    pub fn digest_current_version<A: digest::Digest>(&self) -> Vec<u8> {
        let mut sha3 = A::new();
        sha3.update(&self.current_version());
        sha3.finalize().to_vec()
    }

    pub fn digest<A: digest::Digest>(&self) -> Vec<u8> {
        xor(
            &self.digest_anterior_version::<A>().to_vec(),
            &self.digest_current_version::<A>().to_vec(),
        )
    }

    pub fn render(&self) -> String {
        let mut chunks = Vec::<String>::new();

        for (index, chunk) in match self.settings.axis_boundary {
            AxisBoundary::Len(length) => self
                .sequence
                .clone()
                .chunks(length)
                .map(|units| units.iter().map(|unit| unit.clone()).collect::<Vec<DiffUnit>>())
                .collect::<Vec<Vec<DiffUnit>>>(),
        }
        .iter()
        .enumerate()
        {
            let mut temp_chunks = Vec::<Vec<String>>::new();
            let mut changes = false;
            for unit in chunk {
                let anterior = unit.anterior();
                let current = unit.current();
                if anterior != current {
                    changes = true;
                    temp_chunks.push(vec![
                        format!("\x1b[0;31m0x{:02x}\x1b[0m", anterior.unwrap_or_default()),
                        format!("\x1b[0;32m0x{:02x}\x1b[0m", current.unwrap_or_default()),
                    ]);
                } else {
                    temp_chunks
                        .push(vec![format!("0x{:02x}", anterior.or(current).unwrap_or_default())]);
                }
            }
            if changes {
                chunks.push(format!(
                    "\x1b[0;31m-0x{:07x}\x1b[0m {}",
                    index,
                    temp_chunks
                        .iter()
                        .map(|chunk| chunk[0].to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                ));
                chunks.push(format!(
                    "\x1b[0;32m+0x{:07x}\x1b[0m {}",
                    index,
                    temp_chunks
                        .iter()
                        .map(|chunk| if chunk.len() > 1 {
                            chunk[1].to_string()
                        } else {
                            chunk[0].to_string()
                        }
                        .to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                ));
            } else {
                chunks.push(format!(
                    " 0x{:07x} {}",
                    index,
                    temp_chunks
                        .iter()
                        .map(|chunk| chunk.join(""))
                        .collect::<Vec<String>>()
                        .join(" ")
                ));
            }
        }
        chunks.join("\n")
    }
}

pub fn diff(anterior: &[u8], current: &[u8], axis_boundary: AxisBoundary) -> Result<Diff> {
    match axis_boundary {
        AxisBoundary::Len(len) => diff_len(anterior, current, len),
        // AxisBoundary::LineTermination(_) => Err(Error::DecodingError(format!("not implemented"))),
    }
}

pub fn diff_len(anterior: &[u8], current: &[u8], axis_boundary_len: usize) -> Result<Diff> {
    let mut diff = Vec::<DiffUnit>::new();
    for (y, chunk) in zip_chunked(anterior, current, axis_boundary_len).iter().enumerate() {
        for (x, (anterior, current)) in chunk.iter().enumerate() {
            let anterior = *anterior;
            let current = *current;
            diff.push(DiffUnit::new(anterior, current, x, y));
        }
    }
    Ok(Diff {
        sequence: diff,
        settings: DiffSettings {
            axis_boundary: AxisBoundary::Len(axis_boundary_len),
        },
    })
}

fn optional_max_bytes(items: &[u8], chunk_size: usize) -> Vec<Option<u8>> {
    let rem = rem(items, chunk_size);
    let mut items = items.iter().map(|byte| Some(*byte)).collect::<Vec<Option<u8>>>();
    for _ in 0..rem {
        items.push(None)
    }
    items
}

fn zip_max_modulus(x: &[u8], y: &[u8], chunk_size: usize) -> Vec<(Option<u8>, Option<u8>)> {
    let max_len = [x.len(), y.len()].iter().max().map(|n| *n).unwrap_or(0);
    let mut x = optional_max_bytes(x, chunk_size);
    if x.len() < max_len {
        for _ in 0..(max_len - x.len()) {
            x.push(None)
        }
    }
    let mut y = optional_max_bytes(y, chunk_size);
    if y.len() < max_len {
        for _ in 0..(max_len - y.len()) {
            y.push(None)
        }
    }
    x.iter()
        .zip(y)
        .map(|(x, y)| (*x, y))
        .collect::<Vec<(Option<u8>, Option<u8>)>>()
}

fn zip_chunked(x: &[u8], y: &[u8], chunk_size: usize) -> Vec<Vec<(Option<u8>, Option<u8>)>> {
    let mut chunked = Vec::<Vec<(Option<u8>, Option<u8>)>>::new();
    for chunk in zip_max_modulus(x, y, chunk_size).chunks(chunk_size) {
        let tchunks = chunk
            .iter()
            .map(|possible| *possible)
            .collect::<Vec<(Option<u8>, Option<u8>)>>();
        chunked.push(tchunks);
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

    #[test]
    fn test_optional_max_bytes() {
        let anterior = vec![
            0x00, 0x01, 0x02, 0x03, 0x10, 0x11, 0x12, 0x13, 0x20, 0x21, 0x23, 0x30, 0x31, 0x32,
            0x33,
        ];

        assert_eq!(
            optional_max_bytes(&anterior, 4),
            vec![
                Some(0x00),
                Some(0x01),
                Some(0x02),
                Some(0x03),
                Some(0x10),
                Some(0x11),
                Some(0x12),
                Some(0x13),
                Some(0x20),
                Some(0x21),
                Some(0x23),
                Some(0x30),
                Some(0x31),
                Some(0x32),
                Some(0x33),
                None,
                None,
                None,
            ],
        )
    }

    #[test]
    fn test_zip_max_modulus() {
        let anterior = vec![0x00, 0x01, 0x02, 0x03, 0x10, 0x11, 0x12];
        let current = vec![0x13, 0x20, 0x21, 0x23, 0x30, 0x31, 0x32, 0x33];

        assert_eq!(
            zip_max_modulus(&anterior, &current, 4),
            vec![
                (Some(0x00), Some(0x13)),
                (Some(0x01), Some(0x20)),
                (Some(0x02), Some(0x21)),
                (Some(0x03), Some(0x23)),
                (Some(0x10), Some(0x30)),
                (Some(0x11), Some(0x31)),
                (Some(0x12), Some(0x32)),
                (None, Some(0x33)),
            ]
        )
    }
    #[test]
    fn test_zip_chunked_max_len() {
        let anterior = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let current = vec![0x00, 0x00, 0x02];
        assert_eq!(
            zip_chunked(&anterior, &current, 2),
            vec![
                vec![(Some(0x00), Some(0x00)), (Some(0x00), Some(0x00))],
                vec![(Some(0x00), Some(0x02)), (Some(0x00), None)],
                vec![(Some(0x00), None), (Some(0x00), None)],
            ]
        );
    }
    #[test]
    fn test_zip_chunked() {
        let anterior = vec![0x00, 0x01, 0x02, 0x03, 0x10, 0x11, 0x12];
        let current = vec![0x13, 0x20, 0x21, 0x23, 0x30, 0x31, 0x32, 0x33];

        assert_eq!(
            zip_chunked(&anterior, &current, 4),
            vec![
                vec![
                    (Some(0x00), Some(0x13)),
                    (Some(0x01), Some(0x20)),
                    (Some(0x02), Some(0x21)),
                    (Some(0x03), Some(0x23)),
                ],
                vec![
                    (Some(0x10), Some(0x30)),
                    (Some(0x11), Some(0x31)),
                    (Some(0x12), Some(0x32)),
                    (None, Some(0x33))
                ]
            ]
        )
    }
}

pub fn xor(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    a.into_iter().zip(b.iter()).map(|(a, b)| a ^ b).collect()
}
