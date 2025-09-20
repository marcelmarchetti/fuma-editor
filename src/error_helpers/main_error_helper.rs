use std::io;
use std::io::stdout;
use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use crate::utils::files::read_file;
use crate::utils::path::get_route;

pub fn try_enter_alternate_screen() -> io::Result<()> {
    if let Err(e) = execute!(stdout(), EnterAlternateScreen) {
        eprintln!("Error entering alternate screen {}", e);
        return Err(e);
    }
    Ok(())
}
pub fn try_enable_raw_mode() -> io::Result<()> {
    if let Err(e) = enable_raw_mode() {
        eprintln!("Error entering raw mode {}", e);
        return Err(e);
    }
    Ok(())
}

pub fn try_read_file() -> io::Result<String> {
    let content = read_file(&get_route());
    if let Err(e) =  content {
        eprintln!("Error reading file {}", e);
        return Err(e);
    }
    content
}