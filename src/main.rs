
mod cli;
mod types;
mod errors;
mod validations;

use validations::validate_args;

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
    compress: Option<u8>,
}

fn main() {

    let args = Args::parse();

    if let Err(e) = validate_args(&args) {
        errors::exit(&e);
    }
}