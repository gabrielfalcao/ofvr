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
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Conf(ConfOpt),
    Commit(CommitOpt),
    Log(LogOpt),
    Diff(DiffOpt),
    Matches(MatchesOpt),
}

#[derive(Args, Debug)]
pub struct ConfOpt {
    #[command(subcommand)]
    pub command: ConfCommand,
}

#[derive(Subcommand, Debug)]
pub enum ConfCommand {
    Get(ConfGetOpt),
    Init(ConfInitOpt),
}
#[derive(Args, Debug)]
pub struct ConfGetOpt {}

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

#[derive(Args, Debug)]
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

#[derive(Args, Debug)]
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
#[derive(Args, Debug)]
pub struct DiffOpt {
    #[arg()]
    pub from_file: Path,

    #[arg(short, long)]
    pub ofvr_state_path: Option<Path>,
}
impl DiffOpt {
    pub fn ofvr_state_path(&self) -> Path {
        self.ofvr_state_path
            .clone()
            .or(Some(self.from_file.with_extension(".ofvr")))
            .unwrap()
    }
}

pub fn go(args: Cli) -> Result<()> {
    let path = args.conf_path();
    match args.command {
        Command::Conf(op) => match op.command {
            ConfCommand::Init(iop) => {
                if !iop.overwrite && path.canonicalize()?.is_file() {
                    eprintln!("{} exists", path);
                    std::process::exit(1);
                }
                let author = Author::new(&iop.author_name(), &iop.author_email());
                let conf = Conf::new(author);
                conf.save_to_file(&path)?;
                println!("initialized {}", path);
            },
            ConfCommand::Get(_) => {
                if !path.canonicalize()?.is_file() {
                    eprintln!("{} does not exist", path);
                    std::process::exit(1);
                }
                let conf = Conf::load_from_file(&path)?;
                println!("{}", toml::to_string(&conf)?);
            },
        },
        Command::Commit(op) => {
            let author = op.commit_author(&path)?;
            let mut ofvr = if op.ofvr_state_path().is_file() {
                OFVRState::from_path(&op.ofvr_state_path()).expect("state from path")
            } else {
                OFVRState::empty(&op.ofvr_state_path(), &author).expect("empty state")
            };
            ofvr.commit(&op.from_file, &author, &op.commit_message).expect("commit");
            let commit = ofvr.latest_commit().expect("latest commit");
            println!("{}", commit.log(&ofvr).expect("log"));
        },
        Command::Matches(op) => {
            let ofvr = if op.ofvr_state_path().is_file() {
                OFVRState::from_path(&op.ofvr_state_path())?
            } else {
                eprintln!("{} is not a file", op.ofvr_state_path());
                std::process::exit(1);
            };
            let plain_bytes = op.from_file.read_bytes()?;
            let current_bytes = match ofvr.latest_commit() {
                Some(commit) => commit.data(&ofvr)?.diff(),
                None => Diff::new(AxisBoundary::default()),
            }
            .current_version();

            if plain_bytes == current_bytes {
                println!("{} matches latest version in {}", op.from_file, op.ofvr_state_path());
            } else {
                panic!("{} mismatch", op.from_file);
            }
        },
        Command::Log(op) => {
            let ofvr = if op.ofvr_state_path().is_file() {
                OFVRState::from_path(&op.ofvr_state_path())?
            } else {
                eprintln!("{} is not a file", op.ofvr_state_path());
                std::process::exit(1);
            };
            for commit in ofvr.commits().iter() {
                println!("{}", commit.log(&ofvr)?);
            }
        },
        Command::Diff(op) => {
            let ofvr = if op.ofvr_state_path().is_file() {
                OFVRState::from_path(&op.ofvr_state_path())?
            } else {
                eprintln!("{} is not a file", op.ofvr_state_path());
                std::process::exit(1);
            };

            let mut diff = match ofvr.latest_commit() {
                Some(commit) => commit.data(&ofvr)?.diff(),
                None => Diff::new(AxisBoundary::default()),
            };
            diff.update(&op.from_file.read_bytes()?)?;
            println!("{}", diff.render());
        },
    }
    Ok(())
}
