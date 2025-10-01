use std::{fmt, io};
use crate::editor::fuma_state::FumaState;
use crate::log_debug;
use crate::utils::direction::Direction;

#[derive(Clone)]
pub struct TextSelected {
    text: String,
    pub(crate) row_start: usize,
    pub(crate) col_start: usize,
    pub(crate) row_end: usize,
    pub(crate) col_end: usize,
    pub first_row: usize,
    pub first_col: usize,
    pub direction: Direction,
}

impl fmt::Display for TextSelected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TextSelected(text: \"{}\", start: ({}, {}), end: ({}, {}). First: start: ({}y,{}x), Direction: {})",
            self.text,
            self.row_start,
            self.col_start,
            self.row_end,
            self.col_end,
            self.first_row,
            self.first_col,
            self.direction
        )
    }
}
impl TextSelected {
    fn new(state: &mut FumaState, direction: Direction) -> io::Result<Self> {
        let (x, y) = (state.cursor.x, state.cursor.y);
        let text_lines = state.get_selected_text( x, y, x, y)?;
        let text = text_lines.join("\n");

        Ok(Self {
            text,
            row_start: state.cursor.y,
            row_end: state.cursor.y,
            col_start: state.cursor.x,
            col_end: state.cursor.x,
            first_col: state.cursor.x,
            first_row: state.cursor.y,
            direction
         })
    }

    fn first_is_after(&self) -> bool {
        self.first_row > self.row_end ||
            (self.first_row == self.row_end && self.first_col > self.col_end)
    }

    fn set_direction(&mut self) {
        if self.direction == Direction::Right {
            if self.first_is_after() {
                self.direction = Direction::Left;
            }
        } else {
            if !self.first_is_after() {
                self.direction = Direction::Right;
            }
        }
    }

    pub fn update(&mut self, state: &mut FumaState) -> io::Result<()> {
        self.set_direction();
        if self.direction == Direction::Right {
            self.row_end = state.cursor.y;
            self.col_end = state.cursor.x;
        } else {
            self.row_start= state.cursor.y;
            self.col_start = state.cursor.x;
        }

        self.order();
        let text_lines = state.get_selected_text(self.col_start, self.row_start, self.col_end, self.row_end)?;
        self.text =  text_lines.join("\n");
        Ok(())
    }

    pub fn order(&mut self) {
        if self.row_start > self.row_end ||
            (self.row_start == self.row_end && self.col_start > self.col_end) {
            std::mem::swap(&mut self.row_start, &mut self.row_end);
            std::mem::swap(&mut self.col_start, &mut self.col_end);
        }
    }
}

impl FumaState {
    pub fn get_selected_text(&mut self, start_col: usize, start_row: usize, end_col: usize, end_row: usize) -> io::Result<Vec<String>> {
        let (start_row, start_col) = self.wrap_result.get_logical_position(start_row, start_col)?;
        let (end_row, end_col) = self.wrap_result.get_logical_position(end_row, end_col)?;

        let mut selected_lines = Vec::new();

        if start_row == end_row {
            if let Some(line) = self.buffer.lines.get(start_row) {
                if start_col < line.len() && end_col <= line.len() && start_col < end_col {
                    if let Some(selected) = line.get(start_col..end_col) {
                        selected_lines.push(selected.to_string());
                    }
                }
            }
        }
        else {
            if let Some(first_line) = self.buffer.lines.get(start_row) {
                if start_col < first_line.len() {
                    selected_lines.push(first_line[start_col..].to_string());
                }
            }

            for row in (start_row + 1)..end_row {
                if let Some(line) = self.buffer.lines.get(row) {
                    selected_lines.push(line.to_string());
                }
            }

            if let Some(last_line) = self.buffer.lines.get(end_row) {
                if end_col <= last_line.len() && end_col > 0 {
                    selected_lines.push(last_line[..end_col].to_string());
                }
            }
        }

        Ok(selected_lines)
    }
    pub fn update_or_create_selection(&mut self, direction: Direction, debug:bool ) -> io::Result<()> {
        match self.selected_text.take() {
            Some(mut selection) => {

                log_debug!("Updated selection");
                selection.update(self)?;
                self.selected_text = Some(selection)
            }
            None => {
                log_debug!("New selection");
                self.selected_text  = Some(TextSelected::new(self, direction)?)
            }
        }
        if debug { log_debug!("{}", self.selected_text.clone().unwrap()); }
        Ok(())
    }

    pub fn delete_selection(&mut self) {
        self.selected_text = None
    }
}


