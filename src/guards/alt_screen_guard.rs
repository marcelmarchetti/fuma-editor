use std::io::stdout;
use crossterm::cursor::Show;
use crossterm::execute;
use crossterm::terminal::{LeaveAlternateScreen};
use crate::editor::screen::clean_screen;
use crate::log_info;

pub struct AltScreenGuard;

impl Drop for AltScreenGuard {
    fn drop(&mut self) {
        let _ = clean_screen();
        let _ = execute!(stdout(), LeaveAlternateScreen, Show);
        log_info!("Guard exited cleanly");
    }
}
