mod cursor;
mod utils;
mod guards;
mod error_helpers;
mod editor;

use std::io;
use crate::error_helpers::main_error_helper::{try_read_file};
use crate::editor::program::program_loop;

fn main() -> io::Result<()> {
    let contents = try_read_file()?;
    program_loop(contents)?;
    Ok(())
}