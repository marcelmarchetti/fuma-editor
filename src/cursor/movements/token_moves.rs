use std::io;
use std::io::Error;
use crate::cursor::cursor::CursorPos;
use crate::{log_error, log_info};
use crate::utils::debug::print_token2;
use crate::utils::direction::Direction;
use crate::utils::tokenizer::Token2;


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

    pub fn get_token(&mut self, direction: Direction) -> Option<Token2> {
        let mut buffer: isize = 0;
        let current_wrap_id = self.wrap_ids.get(self.y).copied();

        loop {
            let col_search = self.x.saturating_add_signed(buffer);

            if let Some(token) = self.tokenized_words.iter().find(|t| {
                // Single-line tokens
                (t.row_start <= self.y && t.row_end >= self.y &&
                    t.col_start <= col_search && t.col_end >= col_search) ||
                    // Multi line tokens
                    (t.row_start < self.y && t.row_end > self.y) ||
                    (t.row_start == self.y && t.col_start <= col_search && t.row_end > self.y) ||
                    (t.row_end == self.y && t.col_end >= col_search && t.row_start < self.y)
            }) {
                self.last_token = token.clone();
                return Some(token.clone());
            }

            buffer += direction.step();
            let next_search_col = self.x as isize + buffer;

            //If it doesn't return a token, we check the direction of the move,
            //and if the next/previous row is part of the same logical line to force (or not) a jump
            if next_search_col >= self.line_lengths[self.y] as isize || next_search_col < 0 {
                match direction {
                    Direction::Right if self.y < self.wrap_ids.len().saturating_sub(1) => {
                        if current_wrap_id == self.wrap_ids.get(self.y + 1).copied() {
                            self.y += 1;
                            self.x = 0;
                            buffer = 0;
                            continue;
                        }
                    },
                    Direction::Left if self.y > 0 => {
                        if current_wrap_id == self.wrap_ids.get(self.y - 1).copied() {
                            self.y -= 1;
                            self.x = self.line_lengths[self.y];
                            buffer = 0;
                            continue;
                        }
                    },
                    _ => ()
                }
                break;
            }
        }
        None
    }

    fn cursor_in_last_token(&self) -> bool {
        let token = &self.last_token;
        let col_start = token.col_start.saturating_sub(1);
        let col_end = token.col_end.saturating_add(1);

        (token.row_start <= self.y && token.row_end >= self.y) &&
            // Single-line
            ((token.row_start == token.row_end &&
                self.x >= col_start &&
                self.x <= col_end) ||
                // Multi line
                ((token.row_start < self.y && token.row_end > self.y) ||
                    (token.row_start == self.y && self.x >= col_start) ||
                    (token.row_end == self.y && self.x <= col_end)))
    }

    pub(crate) fn use_last_token(&self, direction: Direction) -> bool {
        match direction {
            Direction::Right => !self.last_fast_right && self.cursor_in_last_token(),
            Direction::Left => self.last_fast_right && self.cursor_in_last_token(),
        }
    }
    pub fn get_token2(&mut self, direction: Direction) -> io::Result<&Token2> {
        let mut move_buffer = self.x;
        let line_length = self.get_current_line_length();

        while move_buffer >= self.min_x && move_buffer <= line_length{
            for token in self.tokenized_words.iter() {
                if (token.col_start..=token.col_end).contains(&move_buffer) && (token.row_start..=token.row_end).contains(&self.y) ||
                    (token.row_start < self.y && token.row_end > self.y) ||
                    (token.row_start == self.y && token.col_start <= move_buffer && token.row_end > self.y) ||
                    (token.row_end == self.y && token.col_end >= move_buffer && token.row_start < self.y){
                    print_token2(token);
                    return match direction {
                        Direction::Right => Ok(token),
                        Direction::Left => Ok(token),
                    }
                }
            }
            match direction {
                Direction::Right => move_buffer += 1,
                Direction::Left => move_buffer = move_buffer.saturating_sub(1),
            }
        }
        Err(io::Error::new(io::ErrorKind::NotFound, format!("Token not found. Buffer ended at {}. Line length: {}", move_buffer, line_length)))
    }


    pub fn move_by_token2(&mut self, direction: Direction) -> io::Result<()> {
        let token: Token2 = if self.use_last_token(direction) {
           self.last_token.clone()
        } else {
            match self.get_token2(direction) {
                Ok(t) => t.clone(),
                Err(e) => {
                    if self.x == self.get_current_line_length() {
                        log_info!("Token not found. Most likely because EOL. Cursor x: {}, Line length: {}", self.x, self.get_current_line_length());
                        return Ok(());
                    }
                    log_info!("Token not found. Cursor x: {} Cursor y: {}, Direction: {}",self.x,self.y,direction);
                    return Err(io::Error::new(io::ErrorKind::NotFound,
                          format!("Token not found. Cursor x: {} Cursor y: {}, Direction: {}", self.x, self.y, direction
                        ),
                    ));
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