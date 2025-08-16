pub mod errors;
pub mod io;
pub mod models;
pub mod utils;
pub use errors::{Error, Result};
pub use io::read_data;
pub use models::*;
pub use utils::{from_deflate_bytes, to_flate_bytes};

pub mod data;
pub use data::{Data, DataSeq, DataSeqIterator};
pub mod hash;
pub use hash::{keccak256, keccak256_full};

pub mod traits;
pub use traits::{FileSystemBytes, PlainBytes};

pub mod cli;
pub use cli::traits::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};

#[doc(hidden)]
pub mod cli;
#[doc(inline)]
pub use cli::{
    Cli, Command, CommitOpt, ConfCommand, ConfGetOpt, ConfInitOpt, ConfOpt, DiffOpt,
    LogOpt, MatchesOpt,
};
