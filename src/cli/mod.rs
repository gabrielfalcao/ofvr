#[doc(hidden)]
pub mod traits;
#[doc(inline)]
pub use traits::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};

#[doc(hidden)]
pub mod command;
#[doc(inline)]
pub use command::{
    Cli, Command, CommitOpt, ConfCommand, ConfGetOpt, ConfInitOpt, ConfOpt, DiffOpt,
    LogOpt, MatchesOpt,
};
