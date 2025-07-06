// cursor.rs
use std::io;
use crossterm::cursor::{MoveTo, Show};
use crossterm::execute;
use std::io::{stdout, Write};

#[derive(Debug)]
pub struct CursorPos {
    x: usize,           // Posición horizontal (0-based)
    pub(crate) y: usize,           // Posición vertical (0-based)
    last_x: usize,      // Última posición X válida (para movimiento vertical)
    max_y: usize,       // Máxima línea posible
    line_lengths: Vec<usize>, // Longitudes reales de cada línea
    contents: String
}

impl CursorPos {
    pub fn new(contents: &str) -> Self {
        let lines: Vec<&str> = contents.lines().collect();
        let line_lengths = lines.iter().map(|l| l.chars().count()).collect(); // Usamos chars() para contar caracteres Unicode
        let max_y = lines.len().saturating_sub(1);
        let contents:String = contents.to_string();

        Self {
            x: 0,
            y: 0,
            last_x: 0,
            max_y,
            line_lengths,
            contents,
        }
    }

    pub fn move_up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
            self.clamp_x_to_current_line();
        }
    }

    pub fn move_down(&mut self) {
        if self.y < self.max_y {
            self.y += 1;
            self.clamp_x_to_current_line();
        }
    }

    pub fn move_left(&mut self) {
        self.x = self.x.saturating_sub(1);
        self.last_x = self.x;
    }

    pub fn move_right(&mut self) {
        let max_x = self.get_current_line_length();
        if self.x < max_x {
            self.x += 1;
            self.last_x = self.x;
        }
    }

    pub fn move_home(&mut self) {
        self.x = 0;
        self.last_x = self.x;
    }

    pub fn move_end(&mut self) {
        self.x = self.get_current_line_length();
        self.last_x = self.x;
    }

    pub fn refresh(&self) -> io::Result<()> {
        execute!(
            stdout(),
            MoveTo(self.x as u16, self.y as u16),
            Show
        )?;
        stdout().flush()?;
        Ok(())
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