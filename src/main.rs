mod cursor;
mod utils;
mod screen;
mod guards;
mod error_helpers;
mod editor;

use std::io;
use crate::error_helpers::main_error_helper::{try_enable_raw_mode, try_enter_alternate_screen, try_read_file};
use crate::guards::alt_screen_guard::AltScreenGuard;
use  editor::{program_loop};

fn main() -> io::Result<()> {
    try_enter_alternate_screen()?;
    let _guard = AltScreenGuard;
    try_enable_raw_mode()?;
    let contents = try_read_file()?;
    program_loop(contents)?;

    Ok(())
}


/*
fn program_loop(contents: String) -> io::Result<()> {
    let (terminal_cols, _) = crossterm::terminal::size()?;
    let mut wrap_result = wrap_content(&contents, terminal_cols as usize);
    let mut tokenized_words = tokenize_text(&wrap_result.wrapped_text, &wrap_result.wrap_ids, false);

    let mut cursor = CursorPos::new(&wrap_result.wrapped_text, wrap_result.wrap_ids.clone(), tokenized_words);
    
    draw_screen(&wrap_result.wrapped_text, &cursor)?;
    cursor.refresh()?;

    loop {
        if event::poll(Duration::from_millis(16))? {
            match event::read()? {
                Event::Resize(cols, _) => {
                    wrap_result = wrap_content(&contents, cols as usize);
                    let old_cursor_state = (cursor.x, cursor.y, cursor.last_x, cursor.vertical_offset);
                    tokenized_words = tokenize_text(&wrap_result.wrapped_text, &wrap_result.wrap_ids, false);
                    
                    cursor = CursorPos::new(&wrap_result.wrapped_text, wrap_result.wrap_ids.clone(), tokenized_words);
                    (cursor.x, cursor.y, cursor.last_x, cursor.vertical_offset) = old_cursor_state;
                    draw_screen(&wrap_result.wrapped_text, &cursor)?;
                },
                Event::Key(KeyEvent { code, kind: KeyEventKind::Press, modifiers, .. }) => match (code, modifiers){
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => break,
                    (KeyCode::Up, _) if cursor.move_up() => draw_screen(&wrap_result.wrapped_text, &cursor)?,
                    (KeyCode::Down, _) if cursor.move_down() => draw_screen(&wrap_result.wrapped_text, &cursor)?,
                    (KeyCode::Left, KeyModifiers::CONTROL) => cursor.move_token(Direction::Left),
                    (KeyCode::Right, KeyModifiers::CONTROL) => cursor.move_token(Direction::Right),
                    (KeyCode::Left, _) => cursor.move_left(),
                    (KeyCode::Right, _) => cursor.move_right(),
                    (KeyCode::Home, _) => cursor.move_home(),
                    (KeyCode::End, _) => cursor.move_end(),
                    (KeyCode::Char('t'), KeyModifiers::CONTROL) => _ = cursor.get_token_on_cursor(),
                    (KeyCode::Char('t'), KeyModifiers::NONE) => _ = tokenize_text(&wrap_result.wrapped_text, &wrap_result.wrap_ids, true),

                    _ => {}
                },
                _ => {}
            }
            cursor.refresh()?;
        }
    }

    Ok(())
}


 */
