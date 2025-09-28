use std::io;
use crate::cursor::cursor::CursorPos;
use crate::{log_info};
use crate::utils::debug::print_token2;
use crate::utils::direction::Direction;
use crate::utils::tokenizer::Token2;

impl Token2 {
    pub fn contains(&self, row: usize, col: usize) -> bool {
        if self.row_start == self.row_end {
            return self.row_start == row && (self.col_start..=self.col_end).contains(&col);
        }

        if row > self.row_start && row < self.row_end {
            return true;
        }

        if row == self.row_start {
            return col >= self.col_start;
        }

        if row == self.row_end {
            return col <= self.col_end;
        }
        false
    }
}


impl CursorPos {
    pub fn get_token_on_cursor(&self) -> Option<Token2> {
        let token = self.tokenized_words.iter()
            .find(|t| {
                // Single-line tokens
                (t.row_start == self.y && t.row_end == self.y &&
                    t.col_start <= self.x && t.col_end >= self.x) ||
                    // Multi line tokens
                    (t.row_start < self.y && t.row_end > self.y) ||
                    (t.row_start == self.y && t.row_end > self.y && t.col_start <= self.x) ||
                    (t.row_start < self.y && t.row_end == self.y) && t.col_end >= self.x
            });
        if token.is_some() {
            return token.cloned();
        }
        None
    }

    fn cursor_in_last_token(&self) -> bool {
        let token = &self.last_token;
        let col_start = token.col_start.saturating_sub(1);
        let col_end = token.col_end.saturating_add(1);

        (col_start..=col_end).contains(&self.x) && (token.row_start..=token.row_end).contains(&self.y) ||
            (token.row_start < self.y && token.row_end > self.y) ||
            (token.row_start == self.y && col_start <= self.x && token.row_end > self.y) ||
            (token.row_end == self.y && col_end >= self.x && token.row_start < self.y)
    }

    pub(crate) fn use_last_token(&self, direction: Direction) -> bool {
        match direction {
            Direction::Right => !self.last_fast_right && self.cursor_in_last_token(),
            Direction::Left => self.last_fast_right && self.cursor_in_last_token(),
        }
    }
    pub fn get_token2(&mut self, direction: Direction) -> io::Result<&Token2> {
        let mut col = self.x;
        let mut row = self.y;
        let wrap_id = self.wrap_ids[self.y];

        loop {
            let line_length = self.get_line_length(row);
            if let Some(token) = self.tokenized_words.iter().find(|t| t.contains(row, col)) {
                return Ok(token);
            }

            match direction {
                Direction::Right => {
                    if col < line_length {
                        col += 1;
                    } else {
                        row += 1;
                        if row >= self.wrap_ids.len() || self.wrap_ids[row] != wrap_id {
                            break;
                        }
                        col = self.min_x;
                    }
                }
                Direction::Left => {
                    if col > self.min_x {
                        col = col.saturating_sub(1);
                    } else {
                        if row == 0 || self.wrap_ids[row - 1] != wrap_id {
                            break;
                        }
                        row = row.saturating_sub(1);
                        col = self.get_line_length(row);
                    }
                }
            }
        }

        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Token not found. Buffer ended at row {}, col {}", row, col),
        ))
    }



    pub fn move_by_token2(&mut self, direction: Direction) -> io::Result<()> {
        let token: Token2 = if self.use_last_token(direction) {
           self.last_token.clone()
        } else {
            match self.get_token2(direction) {
                Ok(t) => t.clone(),
                Err(e) => {
                    log_info!("Token not found. Cursor x: {} Cursor y: {}, Direction: {}, {}",self.x,self.y,direction, e);
                    return Ok(());
                }
            }
        };

        match direction {
            Direction::Right => {
                self.x = token.col_end + 1;
                self.y = token.row_end;
                self.last_fast_right = true
            },
            Direction::Left => {
                self.x = token.col_start.saturating_sub(1).max(self.min_x);
                self.y = token.row_start;
                self.last_fast_right = false
            },
        }
        self.last_token = token;
        self.last_x = self.x;
        Ok(())
    }
}