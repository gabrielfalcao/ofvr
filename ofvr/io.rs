use crate::{Error, Result};
use iocore::Path;
use pqpfs::Data;

pub fn read_data(data_path: &Path) -> Result<Data> {
    if !data_path.is_file() {
        return Err(Error::IOError(format!("{} does not exist", data_path)));
    }
    Ok(data_path.read_bytes()?.into())
}
