use crate::values::globals::TERMINAL_RIGHT_MARGIN;
use crate::cursor::cursor::CursorPos;
use crate::log_debug;

pub struct TextBuffer {
    pub lines: Vec<String>,
}

impl TextBuffer {
    pub fn from_string(text: String) -> Self {
        let mut lines:Vec<String> = text.lines().map(|l| l.to_string()).collect();
        lines.push(String::new());
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

    pub fn insert_newline(&mut self, line: usize, col: usize, cursor: &CursorPos) {

        let (cols, _) = crossterm::terminal::size().unwrap();
        if line < self.line_count() {
            let current = self.lines[line].clone();
            let (before, after) = current.split_at(col);
            self.lines[line] = before.to_string();
            self.lines.insert(line + 1, after.to_string());

            if (cursor.x == 0 || cursor.x as u16 == cols - TERMINAL_RIGHT_MARGIN as u16) && !before.is_empty() && !after.is_empty() {
                self.lines.insert(line + 1, "".to_string());
            }


        } else {
            self.lines.push(String::new());
        }
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
}
