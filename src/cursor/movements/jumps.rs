use std::io;
use std::io::Error;
use crate::cursor::cursor::CursorPos;
use crate::log_error;
use crate::utils::direction::Direction;

impl CursorPos {
    pub fn move_start(&mut self) {
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
                self.x = self.get_current_line_length() + self.min_x;
                self.last_fast_right = true;
            }
        }

        self.last_x = self.x;

        let actual_token = if self.use_last_token(direction) {
            Some(self.last_token.clone())
        } else {
            self.get_token(direction)
        };

        if let Some(token) = actual_token {
            self.last_token = token;
        } else {
            //TODO: Fix tokenization to be able to enable this error
            //log_error!("Can't get token");
            //return Err(io::Error::new(io::ErrorKind::Other, "Can't get token"));
        }

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