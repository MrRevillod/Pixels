use crate::types::TERM_ERR;


#[derive(Debug)]
pub enum CliError {
    FileDontExist,
    ThePathIsNotAFile,
    InvalidMimeType,
    InvalidQuality,
}

impl CliError {

    pub fn new(&self) -> &str {

        match self {

            CliError::FileDontExist => "The file doesn't exist",
            CliError::ThePathIsNotAFile => "The path is not a file",
            CliError::InvalidMimeType => "The file is not a valid image",
            CliError::InvalidQuality => "The quality must be between 1 and 100",
        }
    }
}

pub fn exit(e: &CliError) {
    eprintln!("\n{} Error: {}", *TERM_ERR, e.new());
    std::process::exit(1);
}