use std::io;
use std::time::Duration;
use crossterm::{event};
use crate::editor::fuma_state::FumaState;
use crate::editor::keymap::{build_keymap, handle_event, ReturnEvent};
use crate::error_helpers::main_error_helper::{try_enable_raw_mode, try_enter_alternate_screen};
use crate::guards::alt_screen_guard::AltScreenGuard;
use crate::{log_error, log_info};
use crate::editor::configuration::configuration::load_config;


pub fn program_loop(contents: String) -> io::Result<()> {
    log_info!("Preparing to run the program");


    let configuration = load_config()?;
    let keymap = build_keymap(&configuration.bindings);
    configuration.apply_configuration();

    let mut state = FumaState::new(&contents)?;

    try_enter_alternate_screen()?;
    let _guard = AltScreenGuard;
    try_enable_raw_mode()?;
    
    state.redraw()?;

    log_info!("Loop started");

    let mut rebuild_loop = false;

    loop {
        if event::poll(Duration::from_millis(16))? {
            let evt = event::read()?;
            let value = handle_event(&contents ,evt, &mut state, &keymap, &configuration.bindings)?;
            if value == ReturnEvent::Quit {
                break;
            }
            if value == ReturnEvent::ReloadConfig {
                rebuild_loop = true;
                break;
            }
            if let Err(e) =  state.cursor.refresh() {
                log_error!("Error: {}", e);
                if let Err(e) = state.redraw_and_refresh() {
                    log_error!("Error: {}", e);
                    return Err(e);
                }
            }
        }
    }
    if rebuild_loop {
        program_loop(contents)?;
    }

    log_info!("Program exited cleanly");
    Ok(())
}