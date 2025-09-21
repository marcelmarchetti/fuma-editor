use std::io;
use std::time::Duration;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crate::editor::fuma_state::FumaState;
use crate::editor::load_config::{load_config, KeyBind, KeysConfiguration};
use crate::utils::direction::Direction;

pub fn program_loop(contents: String) -> io::Result<()> {
    let keys_config = load_config()?;
    let mut state = FumaState::new(contents)?;
    state.redraw()?;

    loop {
        if event::poll(Duration::from_millis(16))? {
            let evt = event::read()?;
            if !handle_event(evt, &mut state, &keys_config)? {
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

fn matches_bind(evt: &KeyEvent, bind: &KeyBind) -> bool {
    evt.code == bind.main_key && evt.modifiers == bind.modifier_key
}


fn handle_event(event: Event, state: &mut FumaState, config: &KeysConfiguration) -> io::Result<bool> {
    config.quit.main_key;
    match event {
        Event::Resize(cols, _) => state.resize_console(cols)?,
        Event::Key(evt @ KeyEvent { kind: KeyEventKind::Press, .. }) => {
            if matches_bind(&evt, &config.quit) {
                return Ok(false);
            }
            else if matches_bind(&evt, &config.move_up) && state.cursor.move_up() {
                state.redraw()?;
            }
            else if matches_bind(&evt, &config.move_down) && state.cursor.move_down() {
                state.redraw()?;
            }
            else if matches_bind(&evt, &config.move_left) {
                state.cursor.move_left();
            }
            else if matches_bind(&evt, &config.move_right) {
                state.cursor.move_right();
            }
            else if matches_bind(&evt, &config.move_to_start) {
                state.cursor.move_start();
            }
            else if matches_bind(&evt, &config.move_to_end) {
                state.cursor.move_end();
            }
            else if matches_bind(&evt, &config.move_token_left) {
                state.cursor.move_by_token(Direction::Left);
            }
            else if matches_bind(&evt, &config.move_token_right) {
                state.cursor.move_by_token(Direction::Right);
            }
            else if matches_bind(&evt, &config.get_token) {
                _ = state.cursor.get_token_on_cursor();
            }
            else if matches_bind(&evt, &config.tokenize_text) {
                state.tokenize_text()?;
            }
        }
        _ => {}
    }
    Ok(true)
}