use clap::{Args, Parser, Subcommand};
use bt_diff::{AxisBoundary, Diff};
use iocore::Path;

use crate::{Author, Conf, FileSystemBytes, OFVRState, Result};

#[derive(Parser, Debug)]
#[command()]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    #[arg(short, long)]
    pub conf_path: Option<Path>,
}
impl Cli {
    pub fn conf_path(&self) -> Path {
        self.conf_path.clone().unwrap_or_else(|| Conf::default_path())
    }
    pub fn main() -> Result<()> {
        let cli = Cli::parse();
        let path = args.conf_path();
        match args.command {
            Command::Conf(op) => match op.command {
                ConfCommand::Init(iop) => {
                },
                ConfCommand::Get(_) => {
                },
            },
            Command::Commit(op) => {
            },
            Command::Matches(op) => {
            },
            Command::Log(op) => {
            },
            Command::Diff(op) => {
            },
        }
        Ok(())
    }
}
