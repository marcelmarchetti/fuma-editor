mod cursor;
mod utils;
mod screen;

use std::io;
use std::io::stdout;
use std::time::{Duration};
use crossterm::{event, execute};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::terminal::{enable_raw_mode};
use cursor::CursorPos;
use utils::path::get_route;
use utils::files::read_file;
use crate::screen::{clean_screen, draw_screen};
use crate::utils::content_wrapper::wrap_content;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    program_loop(read_file(&get_route())?)?;
    clean_screen()?;
    Ok(())
}

fn program_loop(contents: String) -> io::Result<()> {
    let (terminal_cols, _) = crossterm::terminal::size()?;
    let mut wrap_result = wrap_content(&contents, terminal_cols as usize);
    let mut cursor = CursorPos::new(&wrap_result.wrapped_text, wrap_result.wrap_ids.clone(), terminal_cols as usize);

    execute!(
        stdout(),
        crossterm::terminal::EnterAlternateScreen,
        crossterm::cursor::Hide
    )?;

    draw_screen(&wrap_result.wrapped_text, &cursor)?;
    cursor.refresh()?;

    loop {
        if event::poll(Duration::from_millis(16))? {
            match event::read()? {
                Event::Resize(cols, _) => {
                    wrap_result = wrap_content(&contents, cols as usize);
                    let old_cursor_state = (cursor.x, cursor.y, cursor.last_x, cursor.vertical_offset);
                    cursor = CursorPos::new(&wrap_result.wrapped_text, wrap_result.wrap_ids, cols as usize);
                    (cursor.x, cursor.y, cursor.last_x, cursor.vertical_offset) = old_cursor_state;
                    draw_screen(&wrap_result.wrapped_text, &cursor)?;
                },
                Event::Key(KeyEvent { code, kind: KeyEventKind::Press, modifiers, .. }) => match (code, modifiers ){
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => break,
                    (KeyCode::Up, _) if cursor.move_up() => draw_screen(&wrap_result.wrapped_text, &cursor)?,
                    (KeyCode::Down, _) if cursor.move_down() => draw_screen(&wrap_result.wrapped_text, &cursor)?,
                    (KeyCode::Left, _) => cursor.move_left(),
                    (KeyCode::Right, _) => cursor.move_right(),
                    (KeyCode::Home, _) => cursor.move_home(),
                    (KeyCode::End, _) => cursor.move_end(),

                    _ => {}
                },
                _ => {}
            }
            cursor.refresh()?;
        }
    }

    Ok(())
}
