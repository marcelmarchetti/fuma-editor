mod cursor;
mod utils;
mod screen;
use tokio::time::{interval, Duration};
use std::io;
use std::io::stdout;
use crossterm::{event, execute};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::terminal::{enable_raw_mode};
use cursor::CursorPos;
use utils::path::get_route;
use utils::files::read_file;
use crate::screen::{clean_screen, draw_screen};
use crate::utils::content_wrapper::wrap_content;
use crate::utils::tokenizer::{ tokenize_text};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    program_loop(read_file(&get_route())?).await?;
    clean_screen()?;
    Ok(())
}

async fn program_loop(contents: String) -> io::Result<()> {
    let (mut cols,mut rows) = crossterm::terminal::size()?;
    let mut wrap_result = wrap_content(&contents, cols as usize);
    let mut tokenized_words = tokenize_text(&wrap_result.wrapped_text);
    let mut interval = interval(Duration::from_millis(200));
    let mut cursor = CursorPos::new(&wrap_result.wrapped_text, wrap_result.wrap_ids.clone(), tokenized_words);

    execute!(
        stdout(),
        crossterm::terminal::EnterAlternateScreen,
        crossterm::cursor::Hide
    )?;
    
    draw_screen(&wrap_result.wrapped_text, &cursor)?;
    cursor.refresh()?;
    

    loop {
        tokio::select! {
        _ = interval.tick() => {

            let (cols_actual, rows_actual) = crossterm::terminal::size()?;
            if cols != cols_actual || rows != rows_actual {
                wrap_result = wrap_content(&contents, cols_actual as usize);
                let old_cursor_state = (cursor.x, cursor.y, cursor.last_x, cursor.vertical_offset);
                tokenized_words = tokenize_text(&wrap_result.wrapped_text);

                cursor = CursorPos::new(&wrap_result.wrapped_text, wrap_result.wrap_ids.clone(), tokenized_words);
                (cursor.x, cursor.y, cursor.last_x, cursor.vertical_offset) = old_cursor_state;
                draw_screen(&wrap_result.wrapped_text, &cursor)?;
                cols = cols_actual;
                rows = rows_actual;
            }
        }
            }
        if event::poll(std::time::Duration::from_millis(16))? {
            match event::read()? {
                Event::Resize(cols_actual, rows_actual) => {
                    wrap_result = wrap_content(&contents, cols_actual as usize);
                    let old_cursor_state = (cursor.x, cursor.y, cursor.last_x, cursor.vertical_offset);
                    tokenized_words = tokenize_text(&wrap_result.wrapped_text);
                    cursor = CursorPos::new(&wrap_result.wrapped_text, wrap_result.wrap_ids.clone(), tokenized_words);
                    (cursor.x, cursor.y, cursor.last_x, cursor.vertical_offset) = old_cursor_state;
                    draw_screen(&wrap_result.wrapped_text, &cursor)?;
                    cols = cols_actual;
                    rows = rows_actual;
                },
                Event::Key(KeyEvent { code, kind: KeyEventKind::Press, modifiers, .. }) => match (code, modifiers){
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => break,
                    (KeyCode::Up, _) if cursor.move_up() => draw_screen(&wrap_result.wrapped_text, &cursor)?,
                    (KeyCode::Down, _) if cursor.move_down() => draw_screen(&wrap_result.wrapped_text, &cursor)?,
                    (KeyCode::Left, KeyModifiers::CONTROL) => cursor.move_word_left(),
                    (KeyCode::Right, KeyModifiers::CONTROL) => cursor.move_word_right(),
                    (KeyCode::Left, _) => cursor.move_left(),
                    (KeyCode::Right, _) => cursor.move_right(),
                    (KeyCode::Home, _) => cursor.move_home(),
                    (KeyCode::End, _) => cursor.move_end(),
                    (KeyCode::Char('t'), _) => _ = tokenize_text(&wrap_result.wrapped_text),

                    _ => {}
                },
                _ => {}
            }
            cursor.refresh()?;
        }
    }

    Ok(())
}
