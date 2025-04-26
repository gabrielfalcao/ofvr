use clap::Parser;
use gdiff::{diff, AxisBoundary, Diff, Result};
use iocore::Path;

pub fn valid_path(val: &str) -> ::std::result::Result<Path, String> {
    let path = Path::new(val);
    if !path.exists() {
        return Err(format!("{} does not exits", &path));
    }
    Ok(path)
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "gdiff command-line utility")]
pub struct Cli {
    #[arg(value_parser = valid_path)]
    pub from: Path,

    #[arg(value_parser = valid_path)]
    pub to: Path,

    #[arg(short, long, default_value = "8")]
    pub boundary_length: usize,
}
impl Cli {
    pub fn diff(&self) -> Result<Diff> {
        let diff = diff(
            &self.from.read_bytes()?,
            &self.to.read_bytes()?,
            AxisBoundary::Len(self.boundary_length),
        )?;
        Ok(diff)
    }
}
fn main() -> Result<()> {
    let args = Cli::parse();
    let diff = args.diff()?;
    println!("\n\x1b[0;31mfrom file: {}\x1b[0m", args.from);
    println!("\x1b[0;32m  to file: {}\x1b[0m\n", args.to);

    println!("{}", diff.render());
    Ok(())
}
