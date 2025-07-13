use clap::Parser;
use std::path::PathBuf;
use toit::layouts::LayoutHandler;

#[derive(Parser)]
struct Args {
    /// Path to root directory
    #[clap(short, long = "dir", default_value = ".")]
    directory: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let lh = LayoutHandler::build(&args.directory.join("layouts"))?;

    println!("{}", lh.get_layout("title").unwrap());

    Ok(())
}
