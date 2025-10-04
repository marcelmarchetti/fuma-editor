use std::io;
use std::io::{stdout, Write};
use std::sync::atomic::Ordering;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::{execute, queue};
use crossterm::style::{ Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{disable_raw_mode, size, BeginSynchronizedUpdate, Clear, ClearType, EndSynchronizedUpdate};
use crate::editor::fuma_state::FumaState;
use crate::log_debug;
use crate::values::colors::{BASE, OVERLAY0, PEACH, SUBTEXT1, TEXT};
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
    let mut out = stdout();
    let (terminal_cols, terminal_rows) = size()?;

    execute!(stdout(), Hide)?;

    execute!(
        stdout(),
        BeginSynchronizedUpdate,
        Clear(ClearType::All),
        SetBackgroundColor(BASE),
    )?;
    let lane_numbering_color = PEACH;
    let text_color = TEXT;


    let lines: Vec<String> = state.wrap_result.wrapped_text.clone();
    let start = state.cursor.vertical_offset;
    let end = (start + terminal_rows as usize).min(lines.len());

    let index_spacing = TERMINAL_LEFT_MARGIN.load(Ordering::Relaxed);

    let effective_cols = terminal_cols as usize - index_spacing;

    let mut first_line = true;
    let mut last_wrapped_inx = 0;

    let mut printed_rows = 0;

    for (i, line) in lines[start..end].iter().enumerate() {
        printed_rows += 1;
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
                queue!(
                    out,
                    SetForegroundColor(lane_numbering_color),
                    MoveTo(0, i as u16),
                    Print(&margin_print))?;
            } else {
                let mut margin = String::new();
                while margin.len() < index_spacing - 1  {
                    margin.push(' ');
                }
                margin.push(DELIMITATOR);
                queue!(
                    out,
                    SetForegroundColor(lane_numbering_color),
                    MoveTo(0, i as u16),
                    Print(&margin))?;
            }
        }

        while line_to_print.len() < effective_cols {
            line_to_print.push(' ');
        }

        queue!(
            out,
            SetForegroundColor(text_color),
            MoveTo(index_spacing as u16, i as u16),
            Print(&line_to_print))?;
    }

    while printed_rows < terminal_rows {
        queue!(
            out,
            MoveTo(0, printed_rows),
            Clear(ClearType::UntilNewLine))?;
        printed_rows += 1;
    }

    queue!(
        out,
        MoveTo(state.cursor.x as u16, (state.cursor.y - state.cursor.vertical_offset) as u16),
        Show,
        EndSynchronizedUpdate,
    )?;
    out.flush()?;

    Ok(())
}

pub fn draw_confirm_message(state: &FumaState, message: &str) -> io::Result<()>{
    let mut out = stdout();
    let (cols, rows) = size()?;

    for col in state.cursor.min_x as u16..=cols {
        queue!(out, MoveTo(col, rows - 1), Print(" "))?;
    }

    queue!(
        out,
        MoveTo(state.cursor.min_x as u16, rows - 1),
        SetBackgroundColor(OVERLAY0),
        SetForegroundColor(SUBTEXT1),
        Hide,
        Print(message),
        ResetColor)?;
    out.flush()?;
    Ok(())
}
