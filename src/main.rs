
mod cli;
mod types;
mod utils;
mod errors;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[clap(version = "0.1", author = "MrRevillod")]
struct Args {
    #[arg(short, long)]
    input: String,
    #[arg(short, long)]
    rename: Option<String>,
    #[arg(short, long)]
    quality: Option<u8>,
}

fn main() {

    let args = Args::parse();

    if let Err(e) = cli::run(&args) {
        errors::exit(&e);
    }
}