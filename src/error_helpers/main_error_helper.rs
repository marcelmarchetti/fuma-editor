use std::{fs, io};
use std::io::stdout;
use std::path::PathBuf;
use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use crate::{log_error, log_info};
use crate::utils::path::get_route;

pub fn try_enter_alternate_screen() -> io::Result<()> {
    if let Err(e) = execute!(stdout(), EnterAlternateScreen) {
        log_error!("Error entering alternate screen {}", e);
        return Err(e);
    }
    Ok(())
}
pub fn try_enable_raw_mode() -> io::Result<()> {
    if let Err(e) = enable_raw_mode() {
        log_error!("Error entering raw mode {}", e);
        return Err(e);
    }
    Ok(())
}


pub fn try_read_file() -> io::Result<String> {
    let path: PathBuf = get_route()?;

    if !path.exists() {
        log_info!("File does not exist: {}", path.to_string_lossy());
        return Ok(String::new());
    }

    match fs::read_to_string(&path) {
        Ok(content) => Ok(content),
        Err(e) => {
            log_error!("Error reading file {}: {}", path.to_string_lossy(), e);
            Err(e)
        }
    }
}
