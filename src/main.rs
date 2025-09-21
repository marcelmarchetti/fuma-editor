mod cursor;
mod utils;
mod screen;
mod guards;
mod error_helpers;
mod editor;

use std::io;
use crate::error_helpers::main_error_helper::{try_enable_raw_mode, try_enter_alternate_screen, try_read_file};
use crate::guards::alt_screen_guard::AltScreenGuard;
use  editor::{program_loop};

fn main() -> io::Result<()> {
    try_enter_alternate_screen()?;
    let _guard = AltScreenGuard;
    try_enable_raw_mode()?;
    let contents = try_read_file()?;
    program_loop(contents)?;

    Ok(())
}