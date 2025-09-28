use std::io;
use crossterm::cursor::{MoveTo, Show};
use crossterm::execute;
use std::io::{stdout, Write};
use std::sync::atomic::Ordering;
use crate::utils::tokenizer::{Token2};
use crate::values::globals::TERMINAL_LEFT_MARGIN;

pub struct CursorPos {
    pub(crate) min_x: usize,
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) last_x: usize,
    pub(crate) max_y: usize,
    pub(crate) line_lengths: Vec<usize>, 
    pub(crate) vertical_offset: usize,
    pub(crate) wrap_ids: Vec<usize>,
    pub(crate) tokenized_words: Vec<Token2>,
    pub(crate) last_token: Token2,
    pub(crate) last_fast_right: bool,
    pub wrapped_text: Vec<String>
}

impl CursorPos {
    pub fn new(wrapped_text: &Vec<String>, wrap_ids: Vec<usize>, tokenized_words: Vec<Token2>) -> Self {
        let lines: &Vec<String> = wrapped_text;
        let max_y = lines.len().saturating_sub(1);
        let min_x = TERMINAL_LEFT_MARGIN.load(Ordering::Relaxed);
        let line_lengths = lines.iter().map(|l| l.chars().count() + min_x).collect();

        let last_token = if tokenized_words.is_empty() {
            Token2::empty()
        } else {
            tokenized_words[0].clone()
        };

        Self {
            min_x,
            x: min_x,
            y: 0,
            last_x: min_x,
            max_y,
            line_lengths,
            vertical_offset: 0,
            wrap_ids,
            tokenized_words,
            last_token,
            last_fast_right: false,
            wrapped_text: wrapped_text.clone(),
        }
    }

    pub(crate) fn is_same_logical_line(&self, other_y: usize) -> bool {
        self.wrap_ids.get(other_y) == self.wrap_ids.get(self.y)
    }

    pub(crate) fn wrap_id_for_line(&self, line: usize) -> Option<usize> {
        self.wrap_ids.get(line).copied()
    }

    pub(crate) fn get_line_length(&self, line: usize) -> usize {
        self.line_lengths.get(line).copied().unwrap_or(0)
    }

    pub(crate) fn get_current_line_length(&self) -> usize {
        self.line_lengths.get(self.y).copied().unwrap_or(0)
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
    pub(crate) fn ensure_visible(&mut self) -> io::Result<bool> {
        let (_, rows) = crossterm::terminal::size()?;
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
        Ok(did_scroll)
        
    }

    pub(crate) fn clamp_x_to_current_line(&mut self) {
        let max_x = self.get_current_line_length();
        if self.last_x > max_x {
            self.x = max_x;
        } else {
            self.x = self.last_x;
        }
    }
}
