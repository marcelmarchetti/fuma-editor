mod cursor;
mod utils;
mod screen;

use std::io;
use std::io::stdout;
use std::time::{Duration};
use crossterm::{event, execute};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::terminal::{enable_raw_mode};
use cursor::CursorPos;
use utils::path::get_route;
use utils::files::read_file;
use crate::screen::{clean_screen, draw_screen};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    program_loop(read_file(&get_route())?)?;
    clean_screen()?;
    Ok(())
}

fn program_loop(contents: String) -> io::Result<()> {
    let mut cursor = CursorPos::new(&contents);
    
    execute!(
        stdout(),
        crossterm::terminal::EnterAlternateScreen,
        crossterm::cursor::Hide
    )?;
    
    draw_screen(&contents, &cursor)?;
    cursor.refresh()?;

    loop {
        if event::poll(Duration::from_millis(16))? { // ~60 FPS
            match event::read()? {
                Event::Resize(..) => {
                    draw_screen(&contents, &cursor)?;
                },
                Event::Key(KeyEvent { code, kind: KeyEventKind::Press, .. }) => match code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up if cursor.move_up() => draw_screen(&contents, &cursor)?,
                    KeyCode::Down if cursor.move_down() => draw_screen(&contents, &cursor)?,
                    KeyCode::Left => cursor.move_left(),
                    KeyCode::Right => cursor.move_right(),
                    KeyCode::Home => cursor.move_home(),
                    KeyCode::End => cursor.move_end(),

                    _ => {}
                },
                _ => {}
            }
            cursor.refresh()?;
        }
    }
    Ok(())
}