// main.rs
mod cursor;

use std::{env, io};
use std::fs;
use std::io::stdout;
use crossterm::{event, execute};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::cursor::MoveTo;
use crossterm::style::Print;
use cursor::CursorPos;

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Uso: editor <archivo>");
        disable_raw_mode()?;
        return Ok(());
    }

    let contents = read_file(&args[1])?;
    program_loop(contents)?;

    // Limpieza final
    execute!(
        stdout(),
        MoveTo(0, 0),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
    )?;
    disable_raw_mode()?;
    Ok(())
}

fn read_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path).map_err(|e| {
        eprintln!("No se pudo leer el archivo: {}", e);
        e
    })
}

fn draw_initial_screen(contents: &str) -> io::Result<()> {
    execute!(
        stdout(),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        MoveTo(0, 0)
    )?;

    for (i, line) in contents.lines().enumerate() {
        execute!(
            stdout(),
            MoveTo(0, i as u16),
            Print(line)
        )?;
    }

    Ok(())
}

fn program_loop(contents: String) -> io::Result<()> {
    let mut cursor = CursorPos::new(&contents);
    draw_initial_screen(&contents)?;
    cursor.refresh()?;

    loop {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
                if kind == KeyEventKind::Press {
                    match code {
                        KeyCode::Char('q') => break,
                        KeyCode::Up => cursor.move_up(),
                        KeyCode::Down => cursor.move_down(),
                        KeyCode::Left => cursor.move_left(),
                        KeyCode::Right => cursor.move_right(),
                        KeyCode::Home => cursor.move_home(),
                        KeyCode::End => cursor.move_end(),
                        KeyCode::Delete => println!("Delete pulsado"),
                        KeyCode::Enter => println!("Enter pulsado"),
                        _ => {}
                    }
                    cursor.refresh()?;
                }
            }
        }
    }

    Ok(())
}