use clap::Parser;
use ofvr::{go, Cli, Result};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder().with_max_level(Level::TRACE).finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let args = Cli::parse();
    go(args)
}
