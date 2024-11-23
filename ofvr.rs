use clap::{Args, Parser, Subcommand};
use iocore::Path;
use ofvr::{OFVRState, Result};

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
}

#[derive(Args, Debug)]
pub struct CommitOpt {
    #[arg()]
    pub from_file: Path,

    #[arg(short = 'm', long = "message", env = "OFVR_COMMIT_MESSAGE")]
    pub commit_message: String,

    #[arg(long = "author", env = "OFVR_COMMIT_AUTHOR")]
    commit_author: Option<String>,

    #[arg(short, long)]
    ofvr_state_path: Option<Path>,
}
#[derive(Args, Debug)]
pub struct LogOpt {
    #[arg()]
    ofvr_state_path: Path,
}
impl LogOpt {
    pub fn ofvr_state_path(&self) -> Path {
        self.ofvr_state_path.clone()
    }
}
impl CommitOpt {
    pub fn ofvr_state_path(&self) -> Path {
        self.ofvr_state_path
            .clone()
            .or(Some(self.from_file.with_extension(".ofvr")))
            .unwrap()
    }
    pub fn commit_author(&self) -> String {
        match &self.commit_author {
            Some(author) => author.to_string(),
            None => [std::env::var("USER"), std::env::var("HOSTNAME")]
                .iter()
                .filter(|var| var.is_ok())
                .map(|var| var.clone().unwrap().to_string())
                .collect::<Vec<String>>()
                .join("@"),
        }
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Command::Commit(op) => {
            let (_ofvr, commit) = if op.ofvr_state_path().is_file() {
                let mut state = OFVRState::from_bytes(&op.ofvr_state_path().read_bytes()?)?;
                let commit =
                    state.commit(&op.from_file, &op.commit_author(), &op.commit_message)?;
                (state, commit)
            } else {
                let state = OFVRState::new_with_commit(
                    &op.ofvr_state_path(),
                    &op.commit_author(),
                    &op.commit_message,
                    &op.from_file,
                )?;
                (
                    state.clone(),
                    state.latest_commit().expect("newly created commit"),
                )
            };
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
            for commit in ofvr.commits() {
                println!("Author: {}", &commit.author());
                println!("Date: {}", &commit.date_rfc2822());
                println!("\t{}\n", &commit.message());
            }
        }
    }
    Ok(())
}
