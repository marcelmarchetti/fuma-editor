use crate::cursor::cursor::CursorPos;
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

    pub fn move_start_line(&mut self) -> bool {
        self.x = 0;
        self.last_x = self.x;
        self.last_fast_right = true;
        self.last_token = self.get_token(Direction::Left).unwrap();
        self.ensure_visible()

    }

    pub fn move_end_line(&mut self) -> bool {
        let mut count = 1;
        while self.is_same_logical_line(self.y + count) {
            count += 1;
        }
        self.y += count - 1;
        self.x = self.get_current_line_length();
        self.last_x = self.x;
        self.last_fast_right = false;
        self.last_token = self.get_token(Direction::Left).unwrap();
        self.clamp_x_to_current_line();
        self.ensure_visible()
    }
}