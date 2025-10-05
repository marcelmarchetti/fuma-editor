# FumaEdit — Minimalist Terminal Text Editor

A fast, modular and visually polished **terminal-based text editor** written entirely in **Rust**.  
Designed around a clean TUI architecture and inspired by modern editors — but keeping the simplicity and control of the terminal.

---

## Current State

FumaEdit is under **active development** and already supports:

- Opening and displaying text files
- Custom Catppuccin-like color theme (RGB rendering via Crossterm)
- Scrollable viewport and line numbering
- Text selection
- Copy, paste, cut
- Syntax tokenization groundwork for Rust (in progress)
- Clean screen restore via RAII guard (`AltScreenGuard`)
- Modular architecture: text buffer, cursor, renderer, parser, input handler

> ⚠️ This is still **alpha-quality software**. Expect bugs, missing features, and occasional jank.  
> FumaEdit is built as both a learning project and a testbed for building a syntax-aware TUI editor in Rust.

---

## Architecture Overview

| Module             | Purpose                                                                 |
|--------------------|-------------------------------------------------------------------------|
| `cursor.rs`        | Tracks cursor position, boundaries, and viewport offset                 |
| `text_buffer.rs`   | Core text storage, line editing, and insertion logic                    |
| `renderer.rs`      | Handles terminal drawing, colors, backgrounds, and overlays             |
| `fuma_state.rs`    | Centralized application state                                           |
| `tokenizer.rs`     | Tokenization engine for words and symbols                               |
| `configuration.rs` | Handles applying various configs from config.toml. Supports hot reload! |

---
## Installation & Usage

### Build from Source

```bash
git clone https://github.com/marcelmarchetti/fuma-editor.git
cd fuma-editor
cargo build --release
```

#### Run:
```bash
./target/release/fumaedit yourfile.rs
```
### Development Mode
```bash
cargo run -- yourfile.txt
```
### Install System-wide (optional)

#### Linux / macOS
```bash
sudo cp ./target/release/fumaedit /usr/local/bin/
```
#### Windows
```cmd
mkdir %USERPROFILE%\bin
copy .\target\release\fumaedit.exe %USERPROFILE%\bin\
setx PATH "%PATH%;%USERPROFILE%\bin"
```
#### Now you can launch:
```
fumaedit myfile.rs
```

---

## Planned Features

    Undo / Redo
    
    Syntax highlighting (Rust first, multi-language later)
    
    Search
        
    Distinct tones for syntax groups (keywords, strings, comments, etc.)
    
    Custom colors and themes

    
    
    

---
## Why Rust?

    -Safe and efficient memory model

    -Excellent control over terminal I/O

    -Perfect for low-latency editors

---
## Clean Exit Guarantee

FumaEdit automatically restores your terminal using a RAII-based guard:

```Rust
pub struct AltScreenGuard;

impl Drop for AltScreenGuard {
    fn drop(&mut self) {
        let _ = clean_screen();
        let _ = execute!(stdout(), LeaveAlternateScreen, Show);
    }
}
```
No matter how the editor exits — your terminal stays clean.

---

## Configuration

FumaEdit supports external configuration via a [config.toml](./docs/config.md) file, allowing you to customize **key bindings**, **editor behavior**, **debug tools**, and **theme colors** — all with **hot reload** support.  
You can modify the file while FumaEdit is running and reload it instantly with `Ctrl + R`.

### File Location

By default, `config.toml` should be located in the same directory as the binary, or in your user configuration folder (coming soon).

---

### Bindings

Customize every editor shortcut with simple, human-readable key definitions.

```toml
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
```
All bindings correspond directly to Crossterm key events, so the mapping is intuitive and easy to extend.
You can freely reassign shortcuts — the parser resolves multi-key combinations automatically.

### Editor

Basic editor behavior toggles:
```toml
[editor]
line_numbering = true #Toggles left-side line numbers.
autosave = false #Automatically writes the current buffer to disk when any change is made in it (currently experimental).
```

### Debug

Developer-focused debugging flags. Useful for visualizing internal editor logic during development.
```toml
[debug]
debug_wrapping = false #Prints wrapping and viewport logic information.
debug_tokenizer = false #Displays token generation step-by-step.
debug_selection = false #Shows how selections are handled internally.
```

### Color

Configure your editor’s color scheme by referencing any of the RGB constants defined in [color.rs](./src/values/colors.rs).

```toml
[color]
text_color = "text"
line_numbering_color = "mauve"
background_color = "base"
dialog_color = "overlay0"
dialog_text_color = "subtext1"
```

Fuma automatically maps string identifiers to these values at runtime.
Available options correspond to the predefined color constants listed in [Colors](./docs/colors.md).


### Hot Reload

Any changes made to config.toml can be applied without restarting the editor.
Simply press your configured hot_reload key (default: Ctrl + r) and Fuma will reload your configuration instantly.

    This makes theme and keybinding experimentation extremely fast — perfect for live tweaking your setup.
---
## Philosophy

“Minimal core. Maximum extensibility.”

FumaEdit is built to be hackable, readable, and expandable, not bloated.

Every subsystem (rendering, state, parsing, input) is kept isolated and replaceable.

---
## ⚠️ Disclaimer

This project is still experimental.
If you somehow use it in production or sensible files, you’re legally classified as a chaotic neutral wizard.

Marcel Marchetti