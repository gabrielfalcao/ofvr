use clap::{Args, Parser, Subcommand};
use gdiff::{diff, AxisBoundary};
use iocore::Path;
use ofvr::Result;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "commands-cli")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    New(NewOpt),
    Check(CheckOpt),
}

#[derive(Args, Debug)]
pub struct NewOpt {
    #[arg()]
    pub ofvr_path: Path,

    #[arg(short, long = "from")]
    pub from_file: Path,
}
#[derive(Args, Debug)]
pub struct CheckOpt {
    #[arg()]
    pub ofvr_path: Path,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Command::New(op) => {
            if op.ofvr_path.is_file() {
                eprintln!("\x1b[1;38;5;160m{} already exists\x1b[0m", op.ofvr_path);
                std::process::exit(1);
            }

            let bytes = op.from_file.read_bytes()?;
            let new_bytes = diff(&Vec::new(), &bytes, AxisBoundary::Len(16))?;
            let flate_bytes = new_bytes.to_flate_bytes()?;
            println!(
                "from {} to {} bytes: {}",
                bytes.len(),
                flate_bytes.len(),
                op.ofvr_path
            );
            op.ofvr_path.write(&flate_bytes)?;
            println!("{:#?}", gdiff::Diff::from_deflate_bytes(&flate_bytes));
        }
        Command::Check(op) => {
            if !op.ofvr_path.is_file() {
                eprintln!("\x1b[1;38;5;160m{} does not exist\x1b[0m", op.ofvr_path);
                std::process::exit(1);
            }

            let flate_bytes = op.ofvr_path.read_bytes()?;
            println!("{:#?}", gdiff::Diff::from_deflate_bytes(&flate_bytes));
        }
    }
    Ok(())
}
