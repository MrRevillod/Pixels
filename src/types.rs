
use crossterm::style::Stylize;
use crossterm::style::StyledContent;

use lazy_static::lazy_static;

use crate::errors::CliError;

pub type CliResult<T> = Result<T, CliError>;

pub const MIME_TYPES: [&str; 4] = ["jpg", "jpeg", "png", "webp"];

pub const DEFAULT_QUALITY: u8 = 80;

lazy_static! {
    pub static ref TERM_OK: StyledContent<&'static str> = "[+]".green();
    pub static ref TERM_ERR: StyledContent<&'static str> = "[-]".red();
    pub static ref TERM_INFO: StyledContent<&'static str> = "[*]".cyan();
    pub static ref TERM_KB: StyledContent<&'static str> = "Kb".green();
}
