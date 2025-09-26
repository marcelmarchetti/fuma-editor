mod cursor;
mod utils;
mod guards;
mod error_helpers;
mod editor;
mod values;

use std::io;
use crate::error_helpers::main_error_helper::try_read_file;
use crate::editor::program::program_loop;
use crate::utils::logger::{init_logging, set_panic_hook};

fn main() -> io::Result<()> {
    set_panic_hook();
    init_logging()?;
    let contents = try_read_file()?;
    program_loop(contents)?;
    Ok(())
}