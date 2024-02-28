
use std::env;
use std::path::{PathBuf, Path};

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

pub fn get_output_path(og_path: &PathBuf, rename: &Option<String>) -> PathBuf {
    
    let output_path = og_path.parent().unwrap();
    let filename = rename.clone().unwrap_or_else(|| clean_filename(og_path));

    let renamed = output_path.join(&filename).with_extension("webp");

    if renamed.exists() {
        return create_unique_filename(output_path, filename);
    }

    renamed
}

fn create_unique_filename(output_path: &Path, filename: String) -> PathBuf {
    
    let mut i = 1;

    loop {
        
        let new_name = format!("{}-{}", filename, i);
        let new_path = output_path.join(new_name).with_extension("webp");

        if !new_path.exists() {
            return new_path;
        }

        i += 1;
    }
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

pub fn clean_filename(path: &PathBuf) -> String {
    path.file_stem().unwrap().to_str().unwrap().to_string()
}
