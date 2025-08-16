use clap::{Parser, Subcommand};
use foobar::cli::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};
use foobar::{Error, Exit, Result};
use clap::{Args, Parser, Subcommand};
use bt_diff::{AxisBoundary, Diff};
use iocore::Path;

use crate::{Author, Conf, FileSystemBytes, OFVRState, Result};


#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = "foobar command-line"
)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}
impl Cli {
    pub fn command(&self) -> Command {
        self.command.clone()
    }
}

impl ParserDispatcher<Error> for Cli {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;

        Ok(())
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    Commit(CommitOpt),
}
impl SubcommandDispatcher<Error> for Command {
    fn dispatch(&self) -> Result<()> {
        match self {
            Command::Commit(op) => op.dispatch()?,
        }
        Ok(())
    }
}

#[derive(Parser, Debug, Clone)]
pub struct CommitOpt {
    #[arg()]
    pub from_file: Path,

    #[arg(short = 'm', long = "message", env = "OFVR_COMMIT_MESSAGE")]
    pub commit_message: String,

    #[arg(short, long)]
    pub ofvr_state_path: Option<Path>,
}
impl CommitOpt {
    pub fn ofvr_state_path(&self) -> Path {
        self.ofvr_state_path
            .clone()
            .or_else(|| Some(self.from_file.with_extension(".ofvr")))
            .unwrap()
    }

    pub fn commit_author(&self, conf_path: &Path) -> Result<Author> {
        if !conf_path.exists() {
            eprintln!(
                "{} does not exist. Initialize a new config with `ofvr conf init'",
                &conf_path
            );
            std::process::exit(1);
        }
        let conf = Conf::load_from_file(conf_path)?;
        Ok(conf.author())
    }
}
impl ArgsDispatcher<Error> for CommitOpt {
    fn dispatch(&self) -> Result<()> {
                let author = self.commit_author(&path)?;
                let mut ofvr = if self.ofvr_state_path().is_file() {
                    OFVRState::from_path(&self.ofvr_state_path()).expect("state from path")
                } else {
                    OFVRState::empty(&self.ofvr_state_path(), &author).expect("empty state")
                };
                ofvr.commit(&self.from_file, &author, &self.commit_message).expect("commit");
                let commit = ofvr.latest_commit().expect("latest commit");
                println!("{}", commit.log(&ofvr).expect("log"));

        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
