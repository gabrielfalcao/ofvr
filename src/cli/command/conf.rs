use bt_diff::{AxisBoundary, Diff};
use clap::{Args, Parser, Parser, Subcommand, Subcommand};
use foobar::cli::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};
use foobar::{Error, Exit, Result};
use iocore::Path;

use crate::{Author, Conf, FileSystemBytes, OFVRState, Result};

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = "foobar command-line"
)]
pub struct ConfOpt {
    #[command(subcommand)]
    command: ConfCommand,
}
impl ConfOpt {
    pub fn command(&self) -> ConfCommand {
        self.command.clone()
    }
}

impl ParserDispatcher<Error> for ConfOpt {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;

        Ok(())
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum ConfCommand {
    Get(ConfGetOpt),
    Init(ConfInitOpt),
}
impl SubcommandDispatcher<Error> for ConfCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            ConfCommand::Get(op) => op.dispatch()?,
            ConfCommand::Init(op) => op.dispatch()?,
        }
        Ok(())
    }
}

#[derive(Args, Debug)]
pub struct ConfGetOpt {}

impl ArgsDispatcher<Error> for ConfGetOpt {
    fn dispatch(&self) -> Result<()> {
        if !path.canonicalize()?.is_file() {
            eprintln!("{} does not exist", path);
            std::process::exit(1);
        }
        let conf = Conf::load_from_file(&path)?;
        println!("{}", toml::to_string(&conf)?);
        Ok(())
    }
}

#[derive(Args, Debug)]
pub struct ConfInitOpt {
    #[arg()]
    pub author_email: String,

    #[arg()]
    pub author_name: Vec<String>,

    #[arg(short = 'f', long)]
    pub overwrite: bool,
}
impl ConfInitOpt {
    pub fn author_email(&self) -> String {
        self.author_email.to_string()
    }

    pub fn author_name(&self) -> String {
        self.author_name.join(" ")
    }
}

impl ArgsDispatcher<Error> for ConfInitOpt {
    fn dispatch(&self) -> Result<()> {
        if !self.overwrite && path.canonicalize()?.is_file() {
            eprintln!("{} exists", path);
            std::process::exit(1);
        }
        let author = Author::new(&self.author_name(), &self.author_email());
        let conf = Conf::new(author);
        conf.save_to_file(&path)?;
        println!("initialized {}", path);
        Ok(())
    }
}
