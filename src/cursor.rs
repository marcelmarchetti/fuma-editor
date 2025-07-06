// cursor.rs
use std::io;
use crossterm::cursor::{MoveTo, Show};
use crossterm::execute;
use std::io::{stdout, Write};
use crate::screen::draw_screen;

#[derive(Debug)]
pub struct CursorPos {
    x: usize,           // Posición horizontal (0-based)
    pub(crate) y: usize,           // Posición vertical (0-based)
    last_x: usize,      // Última posición X válida (para movimiento vertical)
    max_y: usize,       // Máxima línea posible
    line_lengths: Vec<usize>, // Longitudes reales de cada línea
    pub(crate) vertical_offset: usize,
}

impl CursorPos {
    pub fn new(contents: &str) -> Self {
        let lines: Vec<&str> = contents.lines().collect();
        let line_lengths = lines.iter().map(|l| l.chars().count()).collect(); // Usamos chars() para contar caracteres Unicode
        let max_y = lines.len().saturating_sub(1);

        Self {
            x: 0,
            y: 0,
            last_x: 0,
            max_y,
            line_lengths,
            vertical_offset: 0
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
        // Convertimos la posición absoluta y a relativa a la pantalla
        let screen_y = self.y.saturating_sub(self.vertical_offset) as u16;

        execute!(
            stdout(),
            MoveTo(self.x as u16, screen_y),
            Show
        )?;
        stdout().flush()?;
        Ok(())
    }

    // Nuevos métodos para manejar el scroll
    fn ensure_visible(&mut self) -> bool {
        let (_, rows) = crossterm::terminal::size().unwrap();
        let visible_rows = rows as usize;
        let mut did_scroll = false;

        // Scroll hacia arriba si el cursor está por encima del viewport
        if self.y < self.vertical_offset {
            self.vertical_offset = self.y;
            did_scroll = true;
        }
        // Scroll hacia abajo si el cursor está por debajo del viewport
        else if self.y >= self.vertical_offset + visible_rows {
            self.vertical_offset = self.y - visible_rows + 1;
            did_scroll = true;
        }
        did_scroll
        
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