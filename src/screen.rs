use std::io;
use std::io::stdout;
use crossterm::cursor::{MoveTo, Show};
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::disable_raw_mode;

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

pub fn draw_initial_screen(contents: &str) -> io::Result<()> {
    execute!(
        stdout(),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        MoveTo(0, 0)
    )?;

    for (i, line) in contents.lines().enumerate() {
        execute!(
            stdout(),
            MoveTo(0, i as u16),
            Print(line)
        )?;
    }

    Ok(())
}

pub fn handle_resize(contents: &str) -> io::Result<()> {
    clean_screen()?;          // Limpia la pantalla (función existente)
    draw_initial_screen(contents)?;  // Redibuja el contenido (función existente)
    Ok(())
}