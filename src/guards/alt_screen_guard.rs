use std::io::stdout;
use crossterm::cursor::Show;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use crate::screen::clean_screen;

pub struct AltScreenGuard;

impl Drop for AltScreenGuard {
    fn drop(&mut self) {
        let _ = clean_screen();
        let _ = execute!(stdout(), LeaveAlternateScreen, Show);
        let _ = disable_raw_mode();
    }
}
