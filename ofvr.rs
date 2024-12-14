use clap::{Args, Parser, Subcommand};
use gdiff::{AxisBoundary, Diff};
use iocore::Path;
use ofvr::{Author, Conf, FileSystemBytes, OFVRState, Result};
use pqpfs::traits::PlainBytes;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "commands-cli")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Conf(ConfOpt),
    Commit(CommitOpt),
    Log(LogOpt),
    Diff(DiffOpt),
}

#[derive(Args, Debug)]
pub struct ConfOpt {
    #[command(subcommand)]
    pub command: ConfCommand,

    #[arg(short, long)]
    conf_path: Option<Path>,
}
impl ConfOpt {
    pub fn conf_path(&self) -> Path {
        self.conf_path.clone().or_else(|| Some(Conf::default_path())).unwrap()
    }
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
    author_email: String,

    author_name: String,
}
impl ConfInitOpt {
    pub fn author_email(&self) -> String {
        self.author_email.to_string()
    }

    pub fn author_name(&self) -> String {
        self.author_name.to_string()
    }
}

#[derive(Args, Debug)]
pub struct CommitOpt {
    #[arg()]
    pub from_file: Path,

    #[arg(short = 'm', long = "message", env = "OFVR_COMMIT_MESSAGE")]
    pub commit_message: String,

    #[arg(short, long)]
    ofvr_state_path: Option<Path>,

    #[arg(short, long)]
    conf_path: Option<Path>,
}
impl CommitOpt {
    pub fn conf_path(&self) -> Path {
        self.conf_path.clone().or_else(|| Some(Conf::default_path())).unwrap().try_canonicalize()
    }

    pub fn ofvr_state_path(&self) -> Path {
        self.ofvr_state_path
            .clone()
            .or(Some(self.from_file.with_extension(".ofvr")))
            .unwrap()
            .try_canonicalize()
    }

    pub fn commit_author(&self) -> Result<Author> {
        let path = self.conf_path();
        if !path.exists() {
            eprintln!("{} does not exist. Initialize a new config with `ofvr conf init'", &path);
            std::process::exit(1);
        }
        let conf = Conf::load_from_file(&path)?;
        Ok(conf.author())
    }
}
#[derive(Args, Debug)]
pub struct LogOpt {
    #[arg()]
    ofvr_state_path: Path,
}
impl LogOpt {
    pub fn ofvr_state_path(&self) -> Path {
        self.ofvr_state_path.try_canonicalize()
    }
}
#[derive(Args, Debug)]
pub struct DiffOpt {
    #[arg()]
    pub from_file: Path,

    #[arg(short, long)]
    ofvr_state_path: Option<Path>,
}
impl DiffOpt {
    pub fn ofvr_state_path(&self) -> Path {
        self.ofvr_state_path
            .clone()
            .or(Some(self.from_file.with_extension(".ofvr")))
            .unwrap()
            .try_canonicalize()
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Command::Conf(op) => {
            let path = op.conf_path();
            match op.command {
                ConfCommand::Init(iop) => {
                    if path.canonicalize()?.is_file() {
                        eprintln!("{} exists", path);
                        std::process::exit(1);
                    }
                    let author = Author::new(&iop.author_name(), &iop.author_email())?;
                    let conf = Conf::new(author);
                    conf.save_to_file(&path)?;
                    println!("{}", path);
                },
                ConfCommand::Get(_) => {
                    if !path.canonicalize()?.is_file() {
                        eprintln!("{} does not exist", path);
                        std::process::exit(1);
                    }
                    let conf = Conf::load_from_file(&path)?;
                    println!("{}", toml::to_string_pretty(&conf)?);
                },
            }
        },
        Command::Commit(op) => {
            let mut ofvr = if op.ofvr_state_path().is_file() {
                OFVRState::from_plain_bytes(&op.ofvr_state_path().read_bytes()?)?
            } else {
                OFVRState::empty(&op.ofvr_state_path(), &op.commit_author()?)?
            };
            ofvr.commit(&op.from_file, &op.commit_author()?, &op.commit_message)?;
            let commit = ofvr.latest_commit().expect("latest commit");
            println!("{}", commit.log(&ofvr)?);
        },
        Command::Log(op) => {
            let ofvr = if op.ofvr_state_path().is_file() {
                OFVRState::from_plain_bytes(&op.ofvr_state_path().read_bytes()?)?
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
                OFVRState::from_plain_bytes(&op.ofvr_state_path().read_bytes()?)?
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
