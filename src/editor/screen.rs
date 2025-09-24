use std::io;
use std::io::{stdout, BufRead};
use crossterm::cursor::{MoveTo, Show};
use crossterm::{execute};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, size, BeginSynchronizedUpdate, Clear, ClearType, EndSynchronizedUpdate};
use crate::cursor::cursor::CursorPos;
use crate::editor::fuma_state::FumaState;

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


pub fn draw_screen(state: &FumaState) -> io::Result<()> {
    let (_, terminal_rows) = size()?;
    
    execute!(stdout(), crossterm::cursor::Hide)?;
    
    execute!(
        stdout(),
        BeginSynchronizedUpdate,
        Clear(ClearType::All),
    )?;

    let lines: Vec<String> = state.wrap_result.wrapped_text.clone();
    let start = state.cursor.vertical_offset;
    let end = (start + terminal_rows as usize).min(lines.len());

    let index_spacing: u16 = (lines.len() as u16).to_string().len() as u16 + 1;

    let mut first_line = true;
    let mut last_wrapped_inx = 0;

    for (i, line) in lines[start..end].iter().enumerate() {

        if last_wrapped_inx != state.wrap_result.wrap_ids[start + i] || first_line {
            last_wrapped_inx = state.wrap_result.wrap_ids[start + i];
            first_line = false;
            execute!(stdout(), MoveTo(0, i as u16), Print(state.wrap_result.wrap_ids[start + i]))?;
        }

        execute!(stdout(), MoveTo(index_spacing, i as u16), Print(line))?;
    }
    
    execute!(
        stdout(),
        MoveTo(state.cursor.x as u16, (state.cursor.y - state.cursor.vertical_offset) as u16),
        Show,
        EndSynchronizedUpdate
    )?;

    Ok(())
}




