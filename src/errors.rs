
#[derive(Debug)]
pub enum CliError {
    FileDontExist,
    ThePathIsNotAFile,
    InvalidMimeType,
}

impl CliError {

    pub fn new(&self) -> &str {

        match self {

            CliError::FileDontExist => "The file doesn't exist",
            CliError::ThePathIsNotAFile => "The path is not a file",
            CliError::InvalidMimeType => "The file is not a valid image",
        }
    }
}

pub fn exit(e: &CliError) {
    eprintln!("Error: {}", e.new());
    std::process::exit(1);
}