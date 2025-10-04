use std::{fs, io};
use std::io::Write;
use crate::values::globals::{PATH, TERMINAL_RIGHT_MARGIN};
use crate::cursor::cursor::CursorPos;
use crate::{log_debug, log_error};

pub struct TextBuffer {
    pub lines: Vec<String>,
}

impl TextBuffer {
    pub fn from_string(text: String) -> Self {
        let mut lines:Vec<String> = text.lines().map(|l| l.to_string()).collect();
        lines.push(String::new());

        if lines.len() == 1 {
            lines.push(String::new());
        }

        Self { lines }
    }

    pub fn to_string(&self) -> String {
        self.lines.join("\n")
    }

    pub fn insert_char(&mut self, line: usize, col: usize, c: char) {
        if let Some(l) = self.lines.get_mut(line) {
            let byte_index = if col == 0 {
                0
            } else {
                l.char_indices()
                    .nth(col)
                    .map(|(i, _)| i)
                    .unwrap_or(l.len())
            };

            l.insert(byte_index, c);
        }
    }

    pub fn remove_char(&mut self, line: usize, col: usize) {
        if let Some(l) = self.lines.get_mut(line) {
            if col < l.chars().count() {
                let byte_index = l.char_indices()
                    .nth(col)
                    .map(|(i, _)| i)
                    .unwrap_or(l.len());

                let char_len = l[byte_index..].chars().next().map(|c| c.len_utf8()).unwrap_or(1);
                l.drain(byte_index..byte_index + char_len);
            }
        }
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn insert_newline(&mut self, line: usize, col: usize, cursor: &CursorPos) -> io::Result<()> {

        let (cols, _) = crossterm::terminal::size()?;
        if line < self.line_count() {

            let  after: String = self.lines[line].split_off(col);
            self.lines.insert(line + 1, after);

            if (cursor.x == 0 || cursor.x as u16 == cols - TERMINAL_RIGHT_MARGIN as u16) && !self.lines[line].is_empty() && !self.lines[line + 1].is_empty() {
                self.lines.insert(line + 1, "".to_string());
            }

        } else {
            self.lines.push(String::new());
        }
        Ok(())
    }

    pub fn backspace(&mut self, line: usize, col: usize) -> bool {
        if line < self.line_count() {
            if col > 0 {
                self.remove_char(line, col - 1);
                return false;
            } else if line > 0 {
                let removed = self.lines.remove(line);
                let prev_line = &mut self.lines[line - 1];
                prev_line.push_str(&removed);
                return true;
            }
        }
        false
    }

    pub fn delete(&mut self, line: usize, col: usize) {
        let line_count = self.line_count();

        if line < line_count {
            let char_count = self.lines[line].chars().count();

            if col < char_count {
                self.remove_char(line, col);

                if line == line_count - 1 && col == 0 {
                    self.lines.push(String::new());
                }
            } else if line + 1 < line_count - 1 {
                let next_line = self.lines.remove(line + 1);
                self.lines[line].push_str(&next_line);
            }
        }
    }

    pub fn delete_line(&mut self, line: usize) {

        if self.line_count() > line {
            self.lines.remove(line);
        }

        log_debug!("delete_line: {}", line);
        log_debug!("count - 1: {}", self.line_count() - 1);
        log_debug!("entra: {}", self.line_count() == 1 || self.line_count() - 1 == line ||  self.line_count() == line );

        if self.line_count() == 1 || self.line_count() - 1 == line ||  self.line_count() == line {
            self.lines.push(String::new());
        }
    }

    pub fn save_to_file(&self) -> io::Result<()> {
        let path = PATH
            .lock()
            .map_err(|_| {
                log_error!("Failed to acquire PATH lock");
                io::Error::new(io::ErrorKind::Other, "Failed to acquire PATH lock")
            })?
            .clone()
            .ok_or_else(|| {
                log_error!("No path set");
                io::Error::new(io::ErrorKind::Other, "No path set")
            })?;
        let content = self.to_string();
        let mut file = fs::File::create(path)?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }

    pub fn delete_selected_text(
        &mut self,
        start_row: usize,
        start_col: usize,
        end_row: usize,
        end_col: usize,
    ) -> io::Result<()> {
        let (start_row, start_col, end_row, end_col) =
            Self::order_coords(start_row, start_col, end_row, end_col);

        if start_row == end_row {
            self.delete_single_line_selection(start_row, start_col, end_col);
        } else {
            self.delete_multi_line_selection(start_row, start_col, end_row, end_col);
        }

        if self.lines.is_empty() {
            self.lines.push(String::new());
        }

        Ok(())
    }

    fn delete_single_line_selection(&mut self, row: usize, start_col: usize, end_col: usize) {
        if let Some(line) = self.lines.get_mut(row) {
            let len = line.chars().count();
            let start_col = start_col.min(len);
            let end_col = end_col.min(len);

            if start_col < end_col {
                let start_byte = line.char_indices().nth(start_col).map(|(i, _)| i).unwrap_or(line.len());
                let end_byte = line.char_indices().nth(end_col).map(|(i, _)| i).unwrap_or(line.len());
                line.drain(start_byte..end_byte);
            }
        }
    }
    fn delete_multi_line_selection(
        &mut self,
        start_row: usize,
        start_col: usize,
        end_row: usize,
        end_col: usize,
    ) {
        if let Some(first_line) = self.lines.get_mut(start_row) {
            let len = first_line.chars().count();
            let start_col = start_col.min(len);
            let start_byte = first_line.char_indices().nth(start_col).map(|(i, _)| i).unwrap_or(first_line.len());
            first_line.drain(start_byte..);
        }

        if let Some(last_line) = self.lines.get_mut(end_row) {
            let len = last_line.chars().count();
            let end_col = end_col.min(len);
            let end_byte = last_line.char_indices().nth(end_col).map(|(i, _)| i).unwrap_or(last_line.len());
            last_line.drain(..end_byte);
        }

        if start_row < end_row && end_row < self.lines.len() {
            let last_line_content = self.lines[end_row].clone();
            if let Some(first_line) = self.lines.get_mut(start_row) {
                first_line.push_str(&last_line_content);
            }

            for _ in start_row + 1..=end_row {
                if start_row + 1 < self.lines.len() {
                    self.lines.remove(start_row + 1);
                }
            }
        }
    }

    pub fn order_coords(
        start_row: usize,
        start_col: usize,
        end_row: usize,
        end_col: usize,
    ) -> (usize, usize, usize, usize) {
        let (mut sr, mut sc, mut er, mut ec) = (start_row, start_col, end_row, end_col);
        if sr > er || (sr == er && sc > ec) {
            std::mem::swap(&mut sr, &mut er);
            std::mem::swap(&mut sc, &mut ec);
        }
        (sr, sc, er, ec)
    }

    pub fn check_if_changed(&self, content: &String) -> bool {
        let original_content = TextBuffer::from_string(content.clone());

        if self.lines.len() != original_content.lines.len() {
            return true;
        }

        for (inx, line) in self.lines.iter().enumerate() {
            if *line != original_content.lines[inx] {
                return true
            }
        }

        false
    }

}

