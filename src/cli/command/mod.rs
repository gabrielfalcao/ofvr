#[doc(hidden)]
pub mod conf;
#[doc(inline)]
pub use conf::{ConfCommand, ConfGetOpt, ConfInitOpt, ConfOpt};

#[doc(hidden)]
pub mod commit;
#[doc(inline)]
pub use commit::{Cli, Command, CommitOpt};

#[doc(hidden)]
pub mod diff;
#[doc(inline)]
pub use diff::DiffOpt;

#[doc(hidden)]
pub mod matches;
#[doc(inline)]
pub use matches::MatchesOpt;

#[doc(hidden)]
pub mod log;
#[doc(inline)]
pub use log::LogOpt;
