use clap::{Args, Parser, Subcommand};
use bt_diff::{AxisBoundary, Diff};
use iocore::Path;

use crate::{Author, Conf, FileSystemBytes, OFVRState, Result};

use clap::{Parser, Subcommand};
use foobar::cli::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};
use foobar::{Error, Exit, Result};

#[derive(Parser, Debug, Clone)]
pub struct MatchesOpt {
    #[arg()]
    pub from_file: Path,

    #[arg(short, long)]
    pub ofvr_state_path: Option<Path>,
}
impl MatchesOpt {
    pub fn ofvr_state_path(&self) -> Path {
        self.ofvr_state_path
            .clone()
            .or_else(|| Some(self.from_file.with_extension(".ofvr")))
            .unwrap()
    }
}
impl ArgsDispatcher<Error> for MatchesOpt {
    fn dispatch(&self) -> Result<()> {
        let ofvr = if self.ofvr_state_path().is_file() {
            OFVRState::from_path(&self.ofvr_state_path())?
        } else {
            eprintln!("{} is not a file", self.ofvr_state_path());
            std::process::exit(1);
        };
        let plain_bytes = self.from_file.read_bytes()?;
        let current_bytes = match ofvr.latest_commit() {
            Some(commit) => commit.data(&ofvr)?.diff(),
            None => Diff::new(AxisBoundary::default()),
        }
        .current_version();

        if plain_bytes == current_bytes {
            println!("{} matches latest version in {}", self.from_file, self.ofvr_state_path());
        } else {
            panic!("{} mismatch", self.from_file);
        }

        Ok(())
    }
}
