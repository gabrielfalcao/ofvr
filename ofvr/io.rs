use crate::{Error, Result};
use gdiff::{diff, AxisBoundary, Diff};
use iocore::Path;

pub fn new_diff(data_path: &Path, diff_path: &Path, axis_boundary: AxisBoundary) -> Result<Diff> {
    if diff_path.is_file() {
        return Err(Error::IOError(format!("{} already exists", diff_path)));
    }
    if !data_path.is_file() {
        return Err(Error::IOError(format!("{} does not exist", data_path)));
    }

    let bytes = data_path.read_bytes()?;
    let new_bytes = diff(&Vec::new(), &bytes, axis_boundary)?;
    let flate_bytes = new_bytes.to_flate_bytes()?;
    diff_path.write(&flate_bytes)?;
    Ok(Diff::from_deflate_bytes(&flate_bytes)?)
}

pub fn read_diff(diff_path: &Path) -> Result<Diff> {
    if !diff_path.is_file() {
        return Err(Error::IOError(format!("{} does not exist", diff_path)));
    }
    let diff = Diff::from_deflate_bytes(&diff_path.read_bytes()?)?;
    Ok(diff)
}

pub fn read_data(data_path: &Path) -> Result<Vec<u8>> {
    if !data_path.is_file() {
        return Err(Error::IOError(format!("{} does not exist", data_path)));
    }
    Ok(data_path.read_bytes()?)
}

pub fn update_diff(data_path: &Path, diff_path: &Path) -> Result<Diff> {
    let mut diff = read_diff(diff_path)?;
    let data = read_data(data_path)?;
    diff.update(&data)?;
    diff_path.write(&diff.to_flate_bytes()?)?;
    Ok(diff)
}
