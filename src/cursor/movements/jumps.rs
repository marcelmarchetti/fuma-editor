use std::io;
use crate::cursor::cursor::CursorPos;
use crate::utils::direction::Direction;

impl CursorPos {
    pub fn move_start(&mut self) {
        if let Some(first_line) = self.wrap_ids.first() {
            self.y = *first_line;
            self.x = self.min_x;
            self.last_x = self.x;
        }
    }



    pub fn move_end(&mut self) {
        if let Some(last_line) = self.wrap_ids.last() {
            self.y = *last_line;
            self.x = self.get_line_length(*last_line);
            self.last_x = self.x;
        }
    }
    

    fn move_line(&mut self, direction: Direction) -> io::Result<bool> {
        let mut count = 1;

        match direction {
            Direction::Left => {
                while self.is_same_logical_line(self.y.saturating_sub(count)) {
                    count += 1;
                    if self.y.saturating_sub(count) == 0 {
                        break;
                    }
                }
                self.y = self.y.saturating_sub(count - 1);
                self.x = self.min_x;
                self.last_fast_right = false;
            }
            Direction::Right => {
                while self.is_same_logical_line(self.y + count) {
                    count += 1;
                }
                self.y += count - 1;
                self.x = self.get_current_line_length();
                self.last_fast_right = true;
            }
        }

        self.last_x = self.x;
        self.clamp_x_to_current_line();
        Ok(self.ensure_visible()?)
    }

    pub fn move_start_line(&mut self) -> io::Result<bool> {
        self.move_line(Direction::Left)
    }

    pub fn move_end_line(&mut self) -> io::Result<bool> {
        self.move_line(Direction::Right)
    }
}