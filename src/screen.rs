use std::io;
use std::io::{stdout};
use crossterm::cursor::{MoveTo, Show};
use crossterm::{execute};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, size, BeginSynchronizedUpdate, Clear, ClearType, EndSynchronizedUpdate};
use crate::cursor::CursorPos;

pub fn clean_screen() -> io::Result<()>{
    execute!(
        stdout(),
        MoveTo(0, 0),
        Clear(ClearType::All),
        Show
    )?;
    disable_raw_mode()?;
    Ok(())
}


pub fn draw_screen(contents: &str, cursor: &CursorPos) -> io::Result<()> {
    let (_, terminal_rows) = size()?;
    
    execute!(stdout(), crossterm::cursor::Hide)?;
    
    execute!(
        stdout(),
        BeginSynchronizedUpdate,
        Clear(ClearType::All),
    )?;

    let lines: Vec<&str> = contents.lines().collect();
    let start = cursor.vertical_offset;
    let end = (start + terminal_rows as usize).min(lines.len());
    
    for (i, line) in lines[start..end].iter().enumerate() {
        execute!(stdout(), MoveTo(0, i as u16), Print(line))?;
    }
    
    execute!(
        stdout(),
        MoveTo(cursor.x as u16, (cursor.y - cursor.vertical_offset) as u16),
        Show,
        EndSynchronizedUpdate
    )?;

    Ok(())
}


pub struct WrapResult {
    pub wrapped_text: String,
    pub wrap_ids: Vec<usize>,
}
pub fn wrap_content(content: &str, width: usize) -> WrapResult {
    let mut result = Vec::new();
    let mut wrap_ids = Vec::new();

    for (logical_idx, line) in content.lines().enumerate() {
        let mut remaining = line;
        let mut first = true;

        while !remaining.is_empty() {
            let chunk: String = remaining.chars().take(width).collect();
            let byte_len = chunk.len();
            remaining = &remaining[byte_len..];

            result.push(chunk);
            wrap_ids.push(logical_idx); // asignamos el id lógico
            first = false;
        }

        // Línea vacía
        if first {
            result.push(String::new());
            wrap_ids.push(logical_idx);
        }
    }

    WrapResult {
        wrapped_text: result.join("\n"),
        wrap_ids,
    }
}

