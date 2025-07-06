use std::io;
use std::io::stdout;
use crossterm::cursor::{MoveTo, Show};
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::ClearType::All;
use crossterm::terminal::{disable_raw_mode, BeginSynchronizedUpdate, EndSynchronizedUpdate};
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


pub fn draw_screen(contents: &str, cursor: &CursorPos) -> io::Result<()> {
    
    execute!(stdout(), crossterm::terminal::DisableLineWrap)?;
    let (_, terminal_rows) = crossterm::terminal::size()?;

    execute!(
        stdout(),
        BeginSynchronizedUpdate,
        crossterm::terminal::Clear(All),
    )?;

    let lines: Vec<&str> = contents.lines().collect();
    let start = cursor.vertical_offset;
    let end = (start + terminal_rows as usize).min(lines.len());

    for (i, line) in lines[start..end].iter().enumerate() {
        execute!(
            stdout(),
            MoveTo(0, i as u16),
            Print(line)
        )?;
    }

    execute!(
        stdout(),
        EndSynchronizedUpdate,  // <-- ¡NUEVO!
    )?;

    Ok(())
}