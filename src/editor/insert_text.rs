use std::io;
use crate::values::globals::TERMINAL_RIGHT_MARGIN;
use crate::editor::fuma_state::FumaState;
use crate::{log_debug, log_error};

impl FumaState {
    pub fn insert_char(&mut self, c: char) -> io::Result<()> {
        let (cols, _) = crossterm::terminal::size()?;
        let (logical_line, logical_column) = self.wrap_result.get_logical_position(self.cursor.y, self.cursor.x)?;

        self.buffer.insert_char(logical_line, logical_column, c);
        self.cursor.x += 1;

        if self.cursor.x >= cols as usize - TERMINAL_RIGHT_MARGIN + 1{
            self.cursor.x = self.cursor.min_x + 1;
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
        self.cursor.x = self.cursor.min_x;
        self.cursor.ensure_visible();
        self.resize_console()?;
        Ok(())
    }

    pub fn backspace(&mut self) -> io::Result<()> {
        let (logical_line, logical_column) = self.wrap_result
            .get_logical_position(self.cursor.y, self.cursor.x)
            .unwrap_or_else(|e| {
                log_error!("Error: {:?}", e);
                (self.buffer.line_count().saturating_sub(1), self.cursor.x)
            });

        if logical_line < self.buffer.lines.len() {
            if logical_column > 0 {
                self.buffer.backspace(logical_line, logical_column);

                (self.cursor.y, self.cursor.x) = self.wrap_result.get_wrapped_position(logical_line, logical_column.saturating_sub(1))?;

            } else if logical_line > 0 {
                let prev_line_len = self.buffer.lines[logical_line - 1].len();
                let joined_line = self.buffer.backspace(logical_line, logical_column);

                if joined_line {
                    (self.cursor.y,  self.cursor.x) = self.wrap_result.get_wrapped_position(logical_line.saturating_sub(1), prev_line_len)?;
                }
            }let (logical_line, logical_column) = self.wrap_result.get_logical_position(self.cursor.y, self.cursor.x)?;
        }

        self.cursor.last_x = self.cursor.x;
        self.cursor.ensure_visible();
        self.resize_console()?;
        Ok(())
    }

    pub fn delete(&mut self) -> io::Result<()> {
        let (logical_line, logical_column) = self.wrap_result.get_logical_position(self.cursor.y, self.cursor.x)?;

        if logical_line < self.buffer.lines.len() {
            let linea_actual_len = self.buffer.lines[logical_line].chars().count();

            if logical_column < linea_actual_len || logical_line + 1 < self.buffer.lines.len() {
                self.buffer.delete(logical_line, logical_column);
                self.resize_console()?;

            }
        }
        self.cursor.ensure_visible();
        Ok(())
    }

    pub fn delete_line(&mut self) -> io::Result<()> {
        let (logical_line, _) = self.wrap_result.get_logical_position(self.cursor.y, self.cursor.x).unwrap_or_else(|e| {
            log_error!("Error: {:?}", e);
            (self.buffer.lines.len(), self.cursor.x)
        });

        self.buffer.delete_line(logical_line);
        self.cursor.x = self.cursor.min_x;
        self.cursor.last_x = self.cursor.x;
        self.cursor.y = self.wrap_result.get_start_line_wrapped(self.cursor.y).unwrap_or_else(|e|
            { self.buffer.line_count() - 1 }
        );


        self.resize_console()?;
        self.cursor.ensure_visible();
        Ok(())
    }
}