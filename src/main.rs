// main.rs
mod cursor;
mod utils;
mod screen;

use std::{env, io};
use std::fs;
use std::io::{stdout};
use crossterm::{event, execute};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::cursor::MoveTo;
use crossterm::style::Print;
use crossterm::terminal::ClearType::All;
use cursor::CursorPos;
use utils::path::get_route;
use utils::files::read_file;
use crate::screen::{clean_screen, draw_screen, handle_resize};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    program_loop(read_file(&get_route())?)?;
    clean_screen()?;
    Ok(())
}
fn program_loop(contents: String) -> io::Result<()> {
    let mut cursor = CursorPos::new(&contents);
    draw_screen(&contents)?;
    cursor.refresh()?;

    loop {
        if event::poll(std::time::Duration::from_millis(100))? {
            let event = event::read()?;
            
            match event {
                Event::Resize(_, _) => {
                    draw_screen(&contents)?;
                    cursor.refresh()?;
                },
                Event::Key(KeyEvent { code, kind, .. }) if kind == KeyEventKind::Press => {
                    match code {
                        KeyCode::Char('q') => break,
                        KeyCode::Up => cursor.move_up(),
                        KeyCode::Down => cursor.move_down(),
                        KeyCode::Left => cursor.move_left(),
                        KeyCode::Right => cursor.move_right(),
                        KeyCode::Home => cursor.move_home(),
                        KeyCode::End => cursor.move_end(),
                        _ => {}
                    }
                    cursor.refresh()?;
                },
                _ => {} // Otros eventos (mouse, etc.)
            }
        }
    }

    Ok(())
}