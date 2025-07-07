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
use crate::screen::{clean_screen, draw_screen, wrap_content};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    program_loop(read_file(&get_route())?)?;
    clean_screen()?;
    Ok(())
}

fn program_loop(contents: String) -> io::Result<()> {
    let (terminal_cols, _) = crossterm::terminal::size()?;
    let mut wrapped_content = wrap_content(&contents,terminal_cols as usize);
    let mut cursor = CursorPos::new(&wrapped_content);
    
    
    execute!(
        stdout(),
        crossterm::terminal::EnterAlternateScreen,
        crossterm::cursor::Hide
    )?;
    
    draw_screen(&wrapped_content, &cursor)?;
    cursor.refresh()?;

    loop {
        if event::poll(Duration::from_millis(16))? {
            match event::read()? {
                Event::Resize(cols, _) => {
                    wrapped_content = wrap_content(&wrapped_content,cols as usize);
                    let old_cursor_state = (cursor.x, cursor.y, cursor.last_x, cursor.vertical_offset);
                    cursor = CursorPos::new(&wrapped_content);
                    (cursor.x, cursor.y, cursor.last_x, cursor.vertical_offset) = old_cursor_state;
                    draw_screen(&wrapped_content, &cursor)?;
                },
                Event::Key(KeyEvent { code, kind: KeyEventKind::Press, .. }) => match code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up if cursor.move_up() => draw_screen(&wrapped_content, &cursor)?,
                    KeyCode::Down if cursor.move_down() => draw_screen(&wrapped_content, &cursor)?,
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