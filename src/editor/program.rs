use std::io;
use std::time::Duration;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crate::utils::direction::Direction;

pub fn program_loop(contents: String) -> io::Result<()> {
    let mut state = crate::editor::fuma_state::FumaState::new(contents)?;
    state.redraw()?;

    loop {
        if event::poll(Duration::from_millis(16))? {
            let evt = event::read()?;
            if !handle_event(evt, &mut state)? {
                break;
            }
            if let Err(e) =  state.cursor.refresh() {
                eprintln!("Failed to refresh cursor: {}", e);
                if let Err(e) = state.redraw_and_refresh() {
                    eprintln!("Failed to draw screen: {}", e);
                    eprintln!("Critical error, closing fuma editor: {}", e);
                    return Err(e)
                }
            }
        }
    }
    Ok(())
}

fn handle_event(event: Event, state: &mut crate::editor::fuma_state::FumaState) -> io::Result<bool> {
    match event {
        Event::Resize(cols, _) => state.resize_console(cols)?,
        Event::Key(KeyEvent { code, kind: KeyEventKind::Press, modifiers, .. }) => match (code, modifiers){
            (KeyCode::Char('q'), KeyModifiers::CONTROL) => return Ok(false),
            (KeyCode::Up, _) if state.cursor.move_up() => state.redraw()?,
            (KeyCode::Down, _) if state.cursor.move_down() => state.redraw()?,
            (KeyCode::Left, KeyModifiers::CONTROL) => state.cursor.move_by_token(Direction::Left),
            (KeyCode::Right, KeyModifiers::CONTROL) => state.cursor.move_by_token(Direction::Right),
            (KeyCode::Left, _) =>state.cursor.move_left(),
            (KeyCode::Right, _) => state.cursor.move_right(),
            (KeyCode::Home, _) => state.cursor.move_start(),
            (KeyCode::End, _) => state.cursor.move_end(),
            (KeyCode::Char('t'), KeyModifiers::CONTROL) => _ = state.cursor.get_token_on_cursor(),
            (KeyCode::Char('t'), KeyModifiers::NONE) => state.tokenize_text()?,

            _ => {}
        },
        _ => {}
    }
    Ok(true)
}