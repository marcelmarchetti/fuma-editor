use std::io;
use std::ops::Index;
use crate::constants::values::TERMINAL_RIGHT_MARGIN;
use crate::editor::fuma_state::FumaState;

impl FumaState {
    pub fn insert_char(&mut self, c: char) -> io::Result<()> {
        let (cols, _) = crossterm::terminal::size()?;
        let (logical_line, logical_column) = self.wrap_result.get_logical_position(self.cursor.y, self.cursor.x)?;

        self.buffer.insert_char(logical_line, logical_column, c);
        self.cursor.x += 1;

        if self.cursor.x >= cols as usize - TERMINAL_RIGHT_MARGIN + 1{
            self.cursor.x = 1;
            self.cursor.y += 1;
        }

        self.cursor.last_x = self.cursor.x;
        self.resize_console()?;
        Ok(())
    }
    pub fn insert_newline(&mut self) -> io::Result<()> {
        let (logical_line, logical_column) = self.wrap_result.get_logical_position(self.cursor.y, self.cursor.x)?;
        self.buffer.insert_newline(logical_line, logical_column, &self.cursor);
        
        self.cursor.y += 1;
        self.cursor.x = 0;
        self.resize_console()?;
        Ok(())
    }


    pub fn backspace(&mut self) -> io::Result<()> {
        let (logical_line, logical_column) = self.wrap_result.get_logical_position(self.cursor.y, self.cursor.x)?;

        if logical_line < self.buffer.lines.len() {
            if logical_column > 0 {
                self.buffer.backspace(logical_line, logical_column);

                (self.cursor.x, self.cursor.y) = self.wrap_result.get_wrapped_position(logical_line, logical_column.saturating_sub(1))?;

            } else if logical_line > 0 {
                let prev_line_len = self.buffer.lines[logical_line - 1].len();
                let joined_line = self.buffer.backspace(logical_line, logical_column);

                if joined_line {
                    (self.cursor.y,  self.cursor.x) = self.wrap_result.get_wrapped_position(logical_line.saturating_sub(1), prev_line_len)?;
                }
            }
        }

        self.cursor.last_x = self.cursor.x;
        self.resize_console()?;
        Ok(())
    }



    pub fn delete(&mut self) -> io::Result<()> {
        let (logical_line, logical_column) = self.wrap_result.get_logical_position(self.cursor.y, self.cursor.x)?;

        if logical_line < self.buffer.lines.len() {
            let linea_actual_len = self.buffer.lines[logical_line].chars().count();

            if logical_column < linea_actual_len {
                self.buffer.delete(logical_line, logical_column);
                self.resize_console()?;

            } else if logical_line + 1 < self.buffer.lines.len() {
                self.buffer.delete(logical_line, logical_column);
                self.resize_console()?;
                let (new_y, new_x) = self.wrap_result.get_wrapped_position(logical_line, linea_actual_len)?;
                self.cursor.y = new_y;
                self.cursor.x = new_x;
            }
        }

        self.cursor.last_x = self.cursor.x;
        Ok(())
    }
}