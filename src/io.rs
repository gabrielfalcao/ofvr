use iocore::Path;

use crate::{Error, Result};

pub fn read_data(data_path: &Path) -> Result<Vec<u8>> {
    if !data_path.is_file() {
        return Err(Error::IOCoreError(iocore::Error::FileSystemError(format!(
            "{} does not exist",
            data_path
        ))));
    }
    Ok(data_path.read_bytes()?)
}
