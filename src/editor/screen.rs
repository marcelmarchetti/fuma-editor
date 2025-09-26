use std::io;
use std::io::{stdout, BufRead};
use std::sync::atomic::Ordering;
use crossterm::cursor::{MoveTo, Show};
use crossterm::{execute};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, size, BeginSynchronizedUpdate, Clear, ClearType, EndSynchronizedUpdate};
use crate::editor::fuma_state::FumaState;
use crate::values::globals::{DELIMITATOR, SHOW_LINE_NUMBERING, TERMINAL_LEFT_MARGIN};

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

    let index_spacing = TERMINAL_LEFT_MARGIN.load(Ordering::Relaxed);

    let mut first_line = true;
    let mut last_wrapped_inx = 0;

    for (i, line) in lines[start..end].iter().enumerate() {

        if SHOW_LINE_NUMBERING.load(Ordering::Relaxed) {
            if last_wrapped_inx != state.wrap_result.wrap_ids[start + i] || first_line {
                last_wrapped_inx = state.wrap_result.wrap_ids[start + i];

                let mut margin_print = state.wrap_result.wrap_ids[start + i].to_string();
                while margin_print.len() < index_spacing - 1  {
                    margin_print.push(' ');
                }

                margin_print.push(DELIMITATOR);
                first_line = false;
                execute!(stdout(), MoveTo(0, i as u16), Print(margin_print))?;
            } else {
                let mut margin = String::new();
                while margin.len() < index_spacing - 1  {
                    margin.push(' ');
                }
                margin.push(DELIMITATOR);
                execute!(stdout(), MoveTo(0, i as u16), Print(margin))?;
            }
        }


        execute!(stdout(), MoveTo(index_spacing as u16, i as u16), Print(line))?;
    }
    
    execute!(
        stdout(),
        MoveTo(state.cursor.x as u16, (state.cursor.y - state.cursor.vertical_offset) as u16),
        Show,
        EndSynchronizedUpdate
    )?;

    Ok(())
}




