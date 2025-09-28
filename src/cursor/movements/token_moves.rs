use std::io;
use crate::cursor::cursor::CursorPos;
use crate::log_error;
use crate::utils::direction::Direction;
use crate::utils::tokenizer::TokenWithPos;


impl CursorPos {
    pub fn get_token_on_cursor(&self) -> Option<TokenWithPos> {
        let token = self.tokenized_words.iter()
            .find(|t| {
                // Single-line tokens
                (t.row_start == Some(self.y) && t.row_end == Some(self.y) &&
                    t.col_start <= Some(self.x) && t.col_end >= Some(self.x)) ||
                    // Multi line tokens
                    (t.row_start < Some(self.y) && t.row_end > Some(self.y)) ||
                    (t.row_start == Some(self.y) && t.row_end > Some(self.y) && t.col_start <= Some(self.x)) ||
                    (t.row_start < Some(self.y) && t.row_end == Some(self.y)) && t.col_end >= Some(self.x)
            });
        if token.is_some() {
            return token.cloned();
        }
        None
    }

    pub fn get_token(&mut self, direction: Direction) -> Option<TokenWithPos> {
        let mut buffer: isize = 0;
        let current_wrap_id = self.wrap_ids.get(self.y).copied();

        loop {
            let col_search = self.x.saturating_add_signed(buffer);

            if let Some(token) = self.tokenized_words.iter().find(|t| {
                // Single-line tokens
                (t.row_start <= Some(self.y) && t.row_end >= Some(self.y) &&
                    t.col_start <= Some(col_search) && t.col_end >= Some(col_search)) ||
                    // Multi line tokens
                    (t.row_start < Some(self.y) && t.row_end > Some(self.y)) ||
                    (t.row_start == Some(self.y) && t.col_start <= Some(col_search) && t.row_end > Some(self.y)) ||
                    (t.row_end == Some(self.y) && t.col_end >= Some(col_search) && t.row_start < Some(self.y))
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
        let col_start = token.col_start.unwrap_or(0).saturating_sub(1);
        let col_end = token.col_end.unwrap_or(0).saturating_add(1);

        (token.row_start <= Some(self.y) && token.row_end >= Some(self.y)) &&
            // Single-line
            ((token.row_start == token.row_end &&
                self.x >= col_start &&
                self.x <= col_end) ||
                // Multi line
                ((token.row_start < Some(self.y) && token.row_end > Some(self.y)) ||
                    (token.row_start == Some(self.y) && self.x >= col_start) ||
                    (token.row_end == Some(self.y) && self.x <= col_end)))
    }

    pub(crate) fn use_last_token(&self, direction: Direction) -> bool {
        match direction {
            Direction::Right => !self.last_fast_right && self.cursor_in_last_token(),
            Direction::Left => self.last_fast_right && self.cursor_in_last_token(),
        }
    }
    pub fn move_by_token(&mut self, direction: Direction) -> io::Result<()> {
        let actual_token: Option<TokenWithPos> = if self.use_last_token(direction) {
            Some(self.last_token.clone())
        } else {
            self.get_token(direction)
        };

        if let Some(token) = actual_token {
            if token.token.is_none() {
                self.x = self.min_x;
            } else {
                match direction {
                    Direction::Right => {
                        self.x = token.col_end
                            .ok_or_else(|| {
                                let error = io::Error::new(io::ErrorKind::Other, "col_end is None");
                                log_error!("{}", error);
                                return error

                            })?
                            .saturating_add(1);
                        self.y = token.row_end
                            .ok_or_else(|| {
                                let error = io::Error::new(io::ErrorKind::Other, "row_end is None");
                                log_error!("{}", error);
                                return error

                            })?;
                        self.last_fast_right = true;
                    }
                    Direction::Left => {
                        self.x = token.col_start
                            .ok_or_else(|| {
                                let error = io::Error::new(io::ErrorKind::Other, "col_start is None");
                                log_error!("{}", error);
                                return error

                            })?
                            .saturating_sub(1)
                            .max(self.min_x);
                        self.y = token.row_start
                            .ok_or_else(|| {
                                let error = io::Error::new(io::ErrorKind::Other, "row_start is None");
                                log_error!("{}", error);
                                return error

                            })?;
                        self.last_fast_right = false;
                    }
                }
            }
            self.last_x = self.x;
            Ok(())
        } else {
            Ok(())
        }
    }
}