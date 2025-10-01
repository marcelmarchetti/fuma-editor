use std::io;
use std::io::{stdout, Cursor};
use std::sync::atomic::Ordering;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::{execute};
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{disable_raw_mode, size, BeginSynchronizedUpdate, Clear, ClearType, EndSynchronizedUpdate};
use crate::editor::fuma_state::FumaState;
use crate::log_debug;
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

    execute!(stdout(), Hide)?;

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
        let global_line_idx = start + i;
        let mut line_to_print = line.clone();

        if let Some(selection) = &state.selected_text {
            let (sel_start_row, sel_start_col, sel_end_row, sel_end_col) = (
                selection.row_start , selection.col_start - index_spacing,
                selection.row_end, selection.col_end - index_spacing
            );

            if global_line_idx >= sel_start_row && global_line_idx <= sel_end_row {
                let line_start = if global_line_idx == sel_start_row { sel_start_col } else { 0 };
                let line_end = if global_line_idx == sel_end_row { sel_end_col } else { line_to_print.len() };

                if line_start < line_end && line_end <= line_to_print.len() {

                    let selected_part = &line_to_print[line_start..line_end];
                    let before_selection = &line_to_print[0..line_start];
                    let after_selection = &line_to_print[line_end..];
                    log_debug!("{}", selected_part);
                    line_to_print = format!(
                        "{}\x1b[7m{}\x1b[0m{}",
                        before_selection, selected_part, after_selection
                    );
                }
            }
        }

        if SHOW_LINE_NUMBERING.load(Ordering::Relaxed) {
            if last_wrapped_inx != state.wrap_result.wrap_ids[global_line_idx] || first_line {
                last_wrapped_inx = state.wrap_result.wrap_ids[global_line_idx];

                let mut margin_print = state.wrap_result.wrap_ids[global_line_idx].to_string();
                while margin_print.len() < index_spacing - 1 {
                    margin_print.push(' ');
                }

                margin_print.push(DELIMITATOR);
                first_line = false;
                execute!(stdout(), MoveTo(0, i as u16), Print(&margin_print))?;
            } else {
                let mut margin = String::new();
                while margin.len() < index_spacing - 1  {
                    margin.push(' ');
                }
                margin.push(DELIMITATOR);
                execute!(stdout(), MoveTo(0, i as u16), Print(&margin))?;
            }
        }

        execute!(stdout(), MoveTo(index_spacing as u16, i as u16), Print(&line_to_print))?;
    }

    execute!(
        stdout(),
        MoveTo(state.cursor.x as u16, (state.cursor.y - state.cursor.vertical_offset) as u16),
        Show,
        EndSynchronizedUpdate
    )?;

    Ok(())
}

pub fn draw_confirm_message(state: &FumaState, message: &str) -> io::Result<()>{
    let (cols, rows) = size()?;

    for col in state.cursor.min_x as u16..=cols {
        execute!(stdout(), MoveTo(col, rows - 1), Print(" "))?;
    }

    execute!(stdout(), MoveTo(state.cursor.min_x as u16, rows - 1), SetBackgroundColor(Color::DarkBlue),
        SetForegroundColor(Color::Black), Hide, Print(message), ResetColor)?;
    Ok(())
}
