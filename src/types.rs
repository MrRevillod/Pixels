
use crate::errors::CliError;

pub type CliResult<T> = Result<T, CliError>;

pub const MIME_TYPES: [&str; 4] = ["jpg", "jpeg", "png", "webp"];

pub const DEFAULT_QUALITY: u8 = 80;