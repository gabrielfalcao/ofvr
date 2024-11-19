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
    New(NewOpt),
    Update(UpdateOpt),
    Check(CheckOpt),
}

#[derive(Args, Debug)]
pub struct NewOpt {
    #[arg()]
    pub diff_path: Path,

    #[arg(short, long = "from")]
    pub from_file: Path,
}

#[derive(Args, Debug)]
pub struct UpdateOpt {
    #[arg()]
    pub diff_path: Path,

    #[arg(short, long = "from")]
    pub from_file: Path,
}

#[derive(Args, Debug)]
pub struct CheckOpt {
    #[arg()]
    pub diff_path: Path,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Command::New(op) => {
            let ofvr = OFVRState::new_diff(&op.from_file, &op.diff_path)?;
            println!("{:#?}", &ofvr);
        }
        Command::Update(op) => {
            let mut ofvr = OFVRState::from_diff_path(&op.diff_path)?;
            ofvr.update_diff_from_data_path(&op.from_file)?;
            println!("{:#?}", &ofvr);
        }
        Command::Check(op) => {
            let ofvr = OFVRState::from_diff_path(&op.diff_path)?;
            println!("{:#?}", &ofvr);
        }
    }
    Ok(())
}
