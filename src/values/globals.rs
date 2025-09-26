use std::sync::atomic::{AtomicBool, AtomicUsize};

pub const DEBUG_WRAPPING: bool = false;
pub const DEBUG_SCREEN_RENDER: bool = false;
pub const DEBUG_TOKENIZER: bool = false;

pub const TERMINAL_RIGHT_MARGIN: usize = 2;

pub static TERMINAL_LEFT_MARGIN: AtomicUsize = AtomicUsize::new(0);

pub static SHOW_LINE_NUMBERING: AtomicBool = AtomicBool::new(true);

pub const DELIMITATOR: char = '│';
