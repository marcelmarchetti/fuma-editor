// main.rs
mod cursor;
mod utils;
mod screen;

use std::{env, io};
use std::fs;
use std::io::{stdout};
use std::time::{Duration, Instant};
use crossterm::{event, execute};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::cursor::MoveTo;
use crossterm::style::Print;
use crossterm::terminal::ClearType::All;
use cursor::CursorPos;
use utils::path::get_route;
use utils::files::read_file;
use crate::screen::{clean_screen, draw_screen};

const MAX_FPS: u32 = 10060;


fn main() -> io::Result<()> {
    enable_raw_mode()?;
    program_loop(read_file(&get_route())?)?;
    clean_screen()?;
    Ok(())
}
fn program_loop(contents: String) -> io::Result<()> {
    let mut cursor = CursorPos::new(&contents);
    let mut last_render = Instant::now();
    let min_frame_time = Duration::from_secs_f32(1.0 / MAX_FPS as f32);
    
    
    draw_screen(&contents, &cursor)?;
    cursor.refresh()?;

    loop {
        if event::poll(std::time::Duration::from_millis(100))? {
            let event = event::read()?;
            
            
            match event {
                Event::Resize(cols, rows) => {
                    draw_screen(&contents, &cursor)?;
                    cursor.refresh()?;
                },
                Event::Key(KeyEvent { code, kind, .. }) if kind == KeyEventKind::Press => {
                    match code {
                        KeyCode::Char('q') => break,
                        KeyCode::Up => {
                            if cursor.move_up() && should_render(&mut last_render, min_frame_time) {
                                draw_screen(&contents, &cursor)?;
                            }
                        },
                        KeyCode::Down => {
                            if cursor.move_down() && should_render(&mut last_render, min_frame_time) {
                                draw_screen(&contents, &cursor)?;
                            }
                        },
                        KeyCode::Left => cursor.move_left(),
                        KeyCode::Right => cursor.move_right(),
                        KeyCode::Home => cursor.move_home(),
                        KeyCode::End => cursor.move_end(),
                        _ => {}
                    }
                    cursor.refresh()?;
                },
                _ => {} // Otros eventos (mouse, etc.)
            }
        }
    }

    Ok(())
}

fn should_render(last_render: &mut Instant, min_frame_time: Duration) -> bool {
    if last_render.elapsed() >= min_frame_time {
        *last_render = Instant::now();
        true
    } else {
        false
    }
}