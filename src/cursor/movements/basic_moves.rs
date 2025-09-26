use std::io;
use crate::cursor::cursor::CursorPos;
use crate::values::globals::TERMINAL_RIGHT_MARGIN;

impl CursorPos {
    
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
        let (cols , _) = crossterm::terminal::size().unwrap();
        let max_x = self.get_current_line_length() + self.min_x;

        if self.x + 1 <= max_x {
            if self.x + 1 == max_x && self.is_same_logical_line(self.y + 1) {
                self.y += 1;
                self.x = self.min_x;
                self.last_x = self.x;
                return;
            } else {
                self.x += 1;
                self.last_x = self.x;
            }
        } else if self.is_same_logical_line(self.y + 1) {
            self.y += 1;
            self.x = self.min_x;
            self.last_x = self.x;
        }
    }
    pub fn move_left(&mut self) {
        if self.x > self.min_x {
            self.x -= 1;
            self.last_x = self.x;
        } else if self.y > 0 && self.is_same_logical_line(self.y - 1) {
            self.y -= 1;
            self.x = self.get_current_line_length() + self.min_x - 1;
            self.last_x = self.x;
        }
    }
}