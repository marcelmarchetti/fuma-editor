use std::io;
use std::ops::Index;
use crate::constants::values::TERMINAL_RIGHT_MARGIN;
use crate::editor::fuma_state::FumaState;

impl FumaState {
    pub fn insert_char(&mut self, c: char) -> io::Result<()> {
        let (cols, _) = crossterm::terminal::size().unwrap();
        let (logical_line, logical_column) = self.wrap_result.get_logical_position(self.cursor.y, self.cursor.x)?;

        self.buffer.insert_char(logical_line, logical_column, c);
        self.cursor.x += 1;
        self.cursor.last_x = self.cursor.x;

        if self.cursor.x >= cols as usize - TERMINAL_RIGHT_MARGIN{
            self.cursor.x = 1;
            self.cursor.y += 1;
        }


        self.resize_console()?;
        Ok(())
    }
    pub fn insert_newline(&mut self) -> io::Result<()> {
        let (line, col) = (self.cursor.y, self.cursor.x);
        self.buffer.insert_newline(line, col);
        self.cursor.y += 1;
        self.cursor.x = 0;
       self.resize_console()?;
        Ok(())
    }

    pub fn backspace(&mut self) -> io::Result<()> {
        let (line, col) = (self.cursor.y, self.cursor.x);
        if col > 0 {
            self.cursor.x -= 1;
        } else if line > 0 {
            self.cursor.y -= 1;
            self.cursor.x = self.buffer.lines[self.cursor.y].len();
        }
        self.buffer.backspace(line, col);
        self.resize_console()?;
        Ok(())
    }

    pub fn delete(&mut self) -> io::Result<()> {
        let (line, col) = (self.cursor.y, self.cursor.x);
        self.buffer.delete(line, col);
        self.resize_console()?;
        Ok(())
    }
}