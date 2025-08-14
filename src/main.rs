use clap::Parser;
use ofvr::{go, Cli, Result};

fn main() -> Result<()> {
    let args = Cli::parse();
    go(args)
}
