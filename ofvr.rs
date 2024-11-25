use clap::{Args, Parser, Subcommand};
use gdiff::{AxisBoundary, Diff};
use iocore::Path;
use ofvr::{Author, Conf, OFVRState, Result};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "commands-cli")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Commit(CommitOpt),
    Log(LogOpt),
    Diff(DiffOpt),
}

#[derive(Args, Debug)]
pub struct CommitOpt {
    #[arg()]
    pub from_file: Path,

    #[arg(short = 'm', long = "message", env = "OFVR_COMMIT_MESSAGE")]
    pub commit_message: String,

    #[arg(short, long)]
    ofvr_state_path: Option<Path>,
}
impl CommitOpt {
    pub fn ofvr_state_path(&self) -> Path {
        self.ofvr_state_path
            .clone()
            .or(Some(self.from_file.with_extension(".ofvr")))
            .unwrap()
            .try_canonicalize()
    }
    pub fn commit_author(&self) -> Result<Author> {
        let conf = Conf::load()?;
        Ok(Author::from_conf(&conf))
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
        Command::Commit(op) => {
            let (_ofvr_, commit) = if op.ofvr_state_path().is_file() {
                let mut state = OFVRState::from_bytes(&op.ofvr_state_path().read_bytes()?)?;
                let commit =
                    state.commit(&op.from_file, &op.commit_author()?, &op.commit_message)?;
                (state, commit)
            } else {
                let state = OFVRState::new_with_commit(
                    &op.ofvr_state_path(),
                    &op.commit_author()?,
                    &op.commit_message,
                    &op.from_file,
                )?;
                (
                    state.clone(),
                    state.latest_commit().expect("newly created commit"),
                )
            };
            println!("Commit: {}", &commit.id()?);
            println!("Author: {}", &commit.author());
            println!("Date: {}", &commit.date_rfc2822());
            println!("\t{}\n", &commit.message());
        }
        Command::Log(op) => {
            let ofvr = if op.ofvr_state_path().is_file() {
                OFVRState::from_bytes(&op.ofvr_state_path().read_bytes()?)?
            } else {
                eprintln!("{} is not a file", op.ofvr_state_path());
                std::process::exit(1);
            };
            for commit in ofvr.commits().iter() {
                println!("Commit: {}", &commit.id()?);
                println!("Author: {}", &commit.author());
                println!("Date: {}", &commit.date_rfc2822());
                println!("\t{}\n", &commit.message());
            }
        }
        Command::Diff(op) => {
            let ofvr = if op.ofvr_state_path().is_file() {
                OFVRState::from_bytes(&op.ofvr_state_path().read_bytes()?)?
            } else {
                eprintln!("{} is not a file", op.ofvr_state_path());
                std::process::exit(1);
            };

            let mut diff = match ofvr.latest_commit() {
                Some(commit) => commit.diff(),
                None => Diff::new(AxisBoundary::default()),
            };
            diff.update(&op.from_file.read_bytes()?)?;
            println!("{}", diff.render());
        }
    }
    Ok(())
}
