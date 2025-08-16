use bt_diff::{AxisBoundary, Diff};
use clap::{Args, Parser, Parser, Subcommand, Subcommand};
use foobar::cli::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};
use foobar::{Error, Exit, Result};
use iocore::Path;

use crate::{Author, Conf, FileSystemBytes, OFVRState, Result};

#[derive(Args, Debug)]
pub struct LogOpt {
    #[arg()]
    pub ofvr_state_path: Path,

}
impl LogOpt {
    pub fn ofvr_state_path(&self) -> Path {
        self.ofvr_state_path.clone()
    }
}

impl ArgsDispatcher<Error> for LogOpt {
    fn dispatch(&self) -> Result<()> {
        let ofvr = if self.ofvr_state_path().is_file() {
            OFVRState::from_path(&self.ofvr_state_path())?
        } else {
            eprintln!("{} is not a file", self.ofvr_state_path());
            std::process::exit(1);
        };
        for commit in ofvr.commits().iter() {
            println!("{}", commit.log(&ofvr)?);
        }

        Ok(())
    }
}
