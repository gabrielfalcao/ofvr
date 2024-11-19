use crate::io::{new_diff, read_data, read_diff};
use crate::{Result};
use gdiff::AxisBoundary;
use gdiff::Diff;
use iocore::Path;
// use pqpfs::Data;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct OFVRState {
    diff: Diff,
    path: Path,
}

impl OFVRState {
    pub fn new(path: Path, diff: Diff) -> OFVRState {
        OFVRState { diff, path }
    }
    pub fn empty(path: &Path) -> OFVRState {
        let diff = Diff::default();
        let path = path.clone();
        OFVRState { diff, path }
    }
    pub fn new_diff(data_path: &Path, diff_path: &Path) -> Result<OFVRState> {
        let diff = new_diff(&data_path, &diff_path, AxisBoundary::Len(16))?;
        let path = diff_path.clone();
        Ok(OFVRState::new(path, diff))
    }
    pub fn update_diff(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        let data = self.diff.update(data)?;
        self.store()?;
        Ok(data)
    }
    pub fn update_diff_from_data_path(&mut self, data_path: &Path) -> Result<Vec<u8>> {
        let data = read_data(data_path)?;
        Ok(self.update_diff(&data)?)
    }
    pub fn store(&self) -> Result<()> {
        self.path.write(&self.diff.to_flate_bytes()?)?;
        Ok(())
    }
    pub fn from_diff_path(path: &Path) -> Result<OFVRState> {
        let diff = read_diff(path)?;
        let path = path.clone();
        Ok(OFVRState::new(path, diff))
    }
}
