use std::io;
use crossterm::cursor::{MoveTo, Show};
use crossterm::execute;
use std::io::{stdout, Write};
use crossterm::style::Print;
use crate::utils::debug::print_bool;
use crate::utils::tokenizer::{TokenWithPos};
use crate::utils::direction::Direction;


pub struct CursorPos {
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) last_x: usize,
    max_y: usize,
    line_lengths: Vec<usize>, 
    pub(crate) vertical_offset: usize,
    wrap_ids: Vec<usize>,
    tokenized_words: Vec<TokenWithPos>,
    last_token: TokenWithPos,
    last_fast_right: bool,
}

impl CursorPos {
    pub fn new(contents: &str, wrap_ids: Vec<usize>, tokenized_words: Vec<TokenWithPos>) -> Self {
        let lines: Vec<&str> = contents.lines().collect();
        let line_lengths = lines.iter().map(|l| l.chars().count()).collect();
        let max_y = lines.len().saturating_sub(1);
        let last_token = tokenized_words[0].clone();

        Self {
            x: 0,
            y: 0,
            last_x: 0,
            max_y,
            line_lengths,
            vertical_offset: 0,
            wrap_ids,
            tokenized_words,
            last_token,
            last_fast_right: false,
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

    fn get_current_line_length(&self) -> usize {
        self.line_lengths.get(self.y).copied().unwrap_or(0)
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

        // Upward scroll
        if self.y < self.vertical_offset {
            self.vertical_offset = self.y;
            did_scroll = true;
        }
        // Downward scroll
        else if self.y >= self.vertical_offset + visible_rows {
            self.vertical_offset = self.y - visible_rows + 1;
            did_scroll = true;
        }
        did_scroll
        
    }

   

    fn clamp_x_to_current_line(&mut self) {
        let max_x = self.get_current_line_length();
        if self.last_x > max_x {
            self.x = max_x;
        } else {
            self.x = self.last_x;
        }
    }
    
    pub fn get_token_on_cursor(& self) -> Option<TokenWithPos>{
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
        if token.is_some(){
            return token.cloned();
        }
        None
    }
    
    fn get_token(&mut self, direction: Direction) -> Option<TokenWithPos> {
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
    
    fn use_last_token(&self, direction: Direction) -> bool {
        match direction {
            Direction::Right => !self.last_fast_right && self.cursor_in_last_token(),
            Direction::Left  =>  self.last_fast_right && self.cursor_in_last_token(),
        }
    }
    pub fn move_token(&mut self, direction: Direction){
        let actual_token:Option<TokenWithPos> = if self.use_last_token(direction) {
            Some(self.last_token.clone())
        } else {
            
            self.get_token(direction)
        };

        if let Some(token) = actual_token {
            if token.token.is_none() {
                self.x = 0;
            } else {
                match direction {
                    Direction::Right => {
                        self.x = token.col_end.unwrap().saturating_add(1);
                        self.y = token.row_end.unwrap();
                        self.last_fast_right = true;
                    }
                    Direction::Left => {
                        self.x = token.col_start.unwrap().saturating_sub(1);
                        self.y = token.row_start.unwrap();
                        self.last_fast_right = false;
                    }
                }
            }
            self.last_x = self.x;
        } else {
            return
        }
    }
}
