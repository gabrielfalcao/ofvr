use clap::{Args, Parser, Subcommand};
use bt_diff::{AxisBoundary, Diff};
use iocore::Path;

use crate::{Author, Conf, FileSystemBytes, OFVRState, Result};

use clap::{Parser, Subcommand};
use foobar::cli::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};
use foobar::{Error, Exit, Result};

#[derive(Parser, Debug, Clone)]
pub struct DiffOpt {
    #[arg()]
    pub from_file: Path,

    #[arg(short, long)]
    pub ofvr_state_path: Option<Path>,
}
impl ArgsDispatcher<Error> for DiffOpt {
    fn dispatch(&self) -> Result<()> {
        let ofvr = if self.ofvr_state_path().is_file() {
            OFVRState::from_path(&self.ofvr_state_path())?
        } else {
            eprintln!("{} is not a file", self.ofvr_state_path());
            std::process::exit(1);
        };

        let mut diff = match ofvr.latest_commit() {
            Some(commit) => commit.data(&ofvr)?.diff(),
            None => Diff::new(AxisBoundary::default()),
        };
        diff.update(&self.from_file.read_bytes()?)?;
        println!("{}", diff.render());


        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
