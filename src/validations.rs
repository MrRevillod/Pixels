
use std::path;
use std::env;
use std::path::PathBuf;

use crate::Args;
use crate::types::*;
use crate::errors::CliError;

pub fn get_path_from_input(path: &str) -> CliResult<PathBuf> {

    let mut path = path::PathBuf::from(path);

    if path.is_dir() {
        return Err(CliError::ThePathIsNotAFile)
    }
    
    if !path.exists() {
        return Err(CliError::FileDontExist)
    }

    if path.is_relative() {
        path = env::current_dir().unwrap().join(path);
    }

    path = path.canonicalize().map_err(|_| CliError::ThePathIsNotAFile)?;

    Ok(path)
}

pub fn validate_mime_type(path: &PathBuf) -> CliResult<()> {

    let ext = path.extension().and_then(std::ffi::OsStr::to_str)
        .ok_or(CliError::InvalidMimeType)?
    ;

    if !MIME_TYPES.contains(&ext) {
        return Err(CliError::InvalidMimeType)
    }

    Ok(())
}

pub fn validate_args(args: &Args) -> CliResult<()> { 

    let img_path = get_path_from_input(&args.input)?;
    validate_mime_type(&img_path)?;

    Ok(())
}

