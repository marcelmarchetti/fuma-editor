use std::io;
use crate::editor::fuma_state::FumaState;

impl FumaState {

    pub fn insert_char(&mut self, c: char) -> io::Result<()> {
        let (cols, _) = crossterm::terminal::size()?;
        let max_width = cols as usize;

        let (line, col) = (self.cursor.y, self.cursor.x);

        if self.cursor.get_current_line_length() >= max_width - 2 {
            if self.cursor.x + 1 < max_width - 2 {
                self.cursor.x += 1;
                self.buffer.insert_char(line, col, c);
            }
            self.buffer.insert_newline(line, col);
            self.cursor.y += 1;
            self.cursor.x = 0;
            self.buffer.insert_char(self.cursor.y, self.cursor.x, c);
            self.cursor.x += 1;
        } else {
            self.buffer.insert_char(line, col, c);
            self.cursor.x += 1;
        }
        self.cursor.last_x = self.cursor.x;
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