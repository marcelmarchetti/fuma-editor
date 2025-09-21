mod cursor;
mod utils;
mod guards;
mod error_helpers;
mod editor;

use std::io;
use crate::editor::load_config::{load_config, test_config};
use crate::error_helpers::main_error_helper::{try_enable_raw_mode, try_enter_alternate_screen, try_read_file};
use crate::guards::alt_screen_guard::AltScreenGuard;
use crate::editor::program::program_loop;

fn main() -> io::Result<()> {
    try_enter_alternate_screen()?;
    let _guard = AltScreenGuard;
    try_enable_raw_mode()?;
    let contents = try_read_file()?;
    program_loop(contents)?;
    
     

    Ok(())
}