use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::Mutex;
use once_cell::sync::Lazy;

pub static DEBUG_WRAPPING: AtomicBool = AtomicBool::new(false);
pub static DEBUG_TOKENIZER: AtomicBool = AtomicBool::new(false);
pub static DEBUG_SELECTION: AtomicBool = AtomicBool::new(false);

pub const TERMINAL_RIGHT_MARGIN: usize = 2;

pub static TERMINAL_LEFT_MARGIN: AtomicUsize = AtomicUsize::new(0);

pub static TERMINAL_NUMBERING_DELIMITATOR_SEPARATION: usize = 1;

pub static SHOW_LINE_NUMBERING: AtomicBool = AtomicBool::new(true);
pub static AUTOSAVE: AtomicBool = AtomicBool::new(false);

pub const DELIMITATOR: char = '│';

pub static PATH: Lazy<Mutex<Option<PathBuf>>> = Lazy::new(|| Mutex::new(None));
