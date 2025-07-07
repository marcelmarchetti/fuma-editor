use std::io;
use crossterm::cursor::{MoveTo, Show};
use crossterm::execute;
use std::io::{stdout, Write};

#[derive(Debug)]
pub struct CursorPos {
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) last_x: usize,
    max_y: usize,
    line_lengths: Vec<usize>, 
    pub(crate) vertical_offset: usize,
    wrap_ids: Vec<usize>,
    line_width: usize
}

impl CursorPos {
    pub fn new(contents: &str, wrap_ids: Vec<usize>, line_width: usize) -> Self {
        let lines: Vec<&str> = contents.lines().collect();
        let line_lengths = lines.iter().map(|l| l.chars().count()).collect();
        let max_y = lines.len().saturating_sub(1);

        Self {
            x: 0,
            y: 0,
            last_x: 0,
            max_y,
            line_lengths,
            vertical_offset: 0,
            wrap_ids,
            line_width
        }
    }

    pub fn move_up(&mut self) -> bool {
        if self.y > 0 {
            self.y -= 1;
            self.clamp_x_to_current_line();
            return self.ensure_visible();

        }
        false
    }

    pub fn move_down(&mut self) -> bool {
        if self.y < self.max_y {
            self.y += 1;
            self.clamp_x_to_current_line();
            return self.ensure_visible()
        }
        false
    }

    pub fn move_right(&mut self) {
        let max_x = self.get_current_line_length();

        if self.x + 1 <= max_x {
            if self.x + 1 == max_x && self.is_same_logical_line(self.y + 1) {
                self.y += 1;
                self.x = 0;
                self.last_x = self.x;
                return;
            } else {
                self.x += 1;
                self.last_x = self.x;
            }
        } else if self.is_same_logical_line(self.y + 1) {
            self.y += 1;
            self.x = 0;
            self.last_x = self.x;
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
            self.last_x = self.x;
        } else if self.y > 0 && self.is_same_logical_line(self.y - 1) {
            self.y -= 1;
            self.x = self.get_current_line_length().saturating_sub(1);
            self.last_x = self.x;
        }
    }

    fn is_same_logical_line(&self, other_y: usize) -> bool {
        self.wrap_ids.get(other_y) == self.wrap_ids.get(self.y)
    }

    fn wrap_id_for_line(&self, line: usize) -> Option<usize> {
        self.wrap_ids.get(line).copied()
    }

    fn get_line_length(&self, line: usize) -> usize {
        self.line_lengths.get(line).copied().unwrap_or(0)
    }



    pub fn move_home(&mut self) {
        if let Some(current_wrap_id) = self.wrap_id_for_line(self.y) {
            if let Some(first_line) = self.wrap_ids.iter().position(|&id| id == current_wrap_id) {
                self.y = first_line;
                self.x = 0;
                self.last_x = self.x;
            }
        } else {
            self.x = 0;
            self.last_x = self.x;
        }
    }


    pub fn move_end(&mut self) {
        if let Some(current_wrap_id) = self.wrap_id_for_line(self.y) {
            if let Some(last_line) = self.wrap_ids.iter().rposition(|&id| id == current_wrap_id) {
                self.y = last_line;
                self.x = self.get_line_length(last_line);
                self.last_x = self.x;
            }
        } else {
            self.x = self.get_current_line_length();
            self.last_x = self.x;
        }
    }


    pub fn refresh(&self) -> io::Result<()> {
        let screen_y = self.y.saturating_sub(self.vertical_offset) as u16;

        execute!(
            stdout(),
            MoveTo(self.x as u16, screen_y),
            Show
        )?;
        stdout().flush()?;
        Ok(())
    }


    fn ensure_visible(&mut self) -> bool {
        let (_, rows) = crossterm::terminal::size().unwrap();
        let visible_rows = rows as usize;
        let mut did_scroll = false;

        // Upwards scroll
        if self.y < self.vertical_offset {
            self.vertical_offset = self.y;
            did_scroll = true;
        }
        // Downwards scroll
        else if self.y >= self.vertical_offset + visible_rows {
            self.vertical_offset = self.y - visible_rows + 1;
            did_scroll = true;
        }
        did_scroll
        
    }

    fn get_current_line_length(&self) -> usize {
        self.line_lengths.get(self.y).copied().unwrap_or(0)
    }

    fn clamp_x_to_current_line(&mut self) {
        let max_x = self.get_current_line_length();
        if self.last_x > max_x {
            self.x = max_x;
        } else {
            self.x = self.last_x;
        }
    }
}