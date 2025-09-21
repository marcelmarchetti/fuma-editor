use std::io;
use std::time::Duration;
use crossterm::event;
use crate::editor::fuma_state::FumaState;
use crate::editor::keybind::{load_config};
use crate::editor::keymap::{build_keymap, handle_event};
use crate::error_helpers::main_error_helper::{try_enable_raw_mode, try_enter_alternate_screen};
use crate::guards::alt_screen_guard::AltScreenGuard;

pub fn program_loop(contents: String) -> io::Result<()> {
    let keys_config = load_config()?;
    let keymap = build_keymap(&keys_config);
    let mut state = FumaState::new(contents)?;

    try_enter_alternate_screen()?;
    let _guard = AltScreenGuard;
    try_enable_raw_mode()?;
    
    state.redraw()?;

    loop {
        if event::poll(Duration::from_millis(16))? {
            let evt = event::read()?;
            if !handle_event(evt, &mut state, &keymap)? {
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