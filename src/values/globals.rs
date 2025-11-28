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
pub const DEFAULT_CONFIG: &str = r#"
[bindings]
quit = "control q"
move_up = "up"
move_down = "down"
move_left = "left"
move_right = "right"
move_to_start = "home"
move_to_end = "end"
move_token_left = "control left"
move_token_right = "control right"
move_start_line = "control h"
move_end_line = "control l"
delete_line = "control d"
save_file = "control s"

copy = "control c"
paste = "control v"
cut = "control x"

select_key = "shift"
hot_reload = "control r"

[editor]
line_numbering = true
autosave = false

[debug]
debug_wrapping = false
debug_tokenizer = false
debug_selection = false

[color]
text_color = "text"
line_numbering_color = "mauve"
background_color = "base"
dialog_color = "overlay0"
dialog_text_color = "subtext1"
"#;
