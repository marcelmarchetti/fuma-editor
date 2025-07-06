use std::io;
use std::io::stdout;
use crossterm::cursor::{MoveTo, Show};
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::ClearType::All;
use crossterm::terminal::disable_raw_mode;
use crate::cursor::CursorPos;

pub fn clean_screen() -> io::Result<()>{
    execute!(
        stdout(),
        MoveTo(0, 0),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        Show
    )?;
    disable_raw_mode()?;
    Ok(())
}

pub fn draw_screen(contents: &str) -> io::Result<()> {
    execute!(stdout(), crossterm::terminal::DisableLineWrap)?;
    execute!(stdout(), crossterm::terminal::ScrollUp(0))?;
    let (_, terminal_rows) = crossterm::terminal::size()?;

    execute!(
        stdout(),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        MoveTo(0, 0)
    )?;
    
    for (i, line) in contents.lines().take(terminal_rows as usize).enumerate() {
        execute!(
            stdout(),
            MoveTo(0, i as u16),
            Print(line)
        )?;
    }

    Ok(())
}

pub fn handle_resize(contents: &String) -> io::Result<()> {
    clean_screen()?;
    draw_screen(contents)?; 
    Ok(())
}