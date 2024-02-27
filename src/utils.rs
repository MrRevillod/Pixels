

use std::env;
use std::path::PathBuf;

use crate::types::*;
use crate::errors::CliError;

pub fn get_path_from_input(path: &str) -> CliResult<PathBuf> {

    let mut path = PathBuf::from(path);

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

pub fn get_path_from_rename(og_path: &PathBuf, rename: &Option<String>) -> PathBuf {

    let mut path = og_path.clone();

    if let Some(rename) = rename {
        let clean_filename = rename.split(".").collect::<Vec<&str>>()[0];
        path.set_file_name(clean_filename);
    }

    path
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

pub fn normalize_quality(quality: Option<u8>) -> CliResult<u8> {

    if quality.is_some_and(|q| q < 1 || q > 100) {
        return Err(CliError::InvalidQuality)        
    }

    Ok(quality.unwrap_or(DEFAULT_QUALITY))
}

