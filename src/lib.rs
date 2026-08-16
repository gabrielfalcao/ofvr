#[doc(hidden)] pub mod errors;
#[doc(inline)] pub use errors::{Error, Result};

#[doc(hidden)] pub mod io;
#[doc(inline)] pub use io::read_data;

#[doc(hidden)] pub mod models;
#[doc(inline)] pub use models::{Author, Commit, CommitData, Conf, ID, OFVRState};

#[doc(hidden)] pub mod utils;
#[doc(inline)]
pub use utils::{chunk_padded, from_deflate_bytes, to_flate_bytes, xor, xor_ip};

#[doc(hidden)] pub mod data;
#[doc(inline)] pub use data::{Data, DataIterator, DataSeq, DataSeqIterator, ToData};

#[doc(hidden)] pub mod hash;
#[doc(inline)]
pub use hash::{Digest, Keccak256, Keccak256Full, keccak256, keccak256_full};

#[doc(hidden)] pub mod traits;
#[doc(inline)] pub use traits::{FileSystemBytes, PlainBytes, Sha3_384};

#[doc(hidden)] pub mod cli;
#[doc(inline)]
pub use cli::{
    Cli,
    Command,
    CommitOpt,
    ConfCommand,
    ConfGetOpt,
    ConfInitOpt,
    ConfOpt,
    DiffOpt,
    LogOpt,
    MatchesOpt,
    go,
};
