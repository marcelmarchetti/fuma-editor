# FumaEdit - Minimal Terminal File Editor

A barebones terminal file editor written in Rust. Currently in early development – it handles basic file viewing only.

## Current State

This is just a proof-of-concept. It currently supports:

- Opening text files via command-line argument
- Displaying file contents in the terminal
- Navigating through the file with arrow keys
- Handling basic terminal resizing

**Warning:** This is pre-alpha software. Many essential features are missing.

---

## Installation and Usage

### Build from Source

Clone the repository:

```bash
  git clone https://github.com/yourusername/fumaedit.git  
  cd fumaedit
```

Build in release mode:

```bash
  cargo build --release
```

The binary will be located at:

```bash
  ./target/release/fumaedit
```

---

### Optional: Install System-Wide

#### On Linux/macOS:

```bash
  sudo cp ./target/release/fumaedit /usr/local/bin/
```

#### On Windows:

```cmd
mkdir %USERPROFILE%\bin
copy .\target\release\fumaedit.exe %USERPROFILE%\bin\
setx PATH "%PATH%;%USERPROFILE%\bin"
```

Once installed, you can run it from anywhere:

```bash
  fumaedit yourfile.txt
```

---

## Path Handling

FumaEdit accepts:

- Absolute paths (e.g., `/home/user/file.txt`, `C:\Users\user\file.txt`)
- Relative paths (e.g., `./file.txt`, `otherdir/file.txt`)
- Home directory shortcuts (e.g., `~/documents/file.txt` on Unix-like systems)

It attempts to resolve paths correctly regardless of format.

---

## Development Usage

To test during development:

```bash
  cargo run -- yourfile.txt
```

---

## Controls

- Arrow keys: Move cursor
- `Home` / `End`: Jump to start / end of line
- `q`: Quit

---

## Technical Details

Built using:

- Crossterm for terminal handling
- Standard Rust I/O
- Basic line-wrapping and viewport management

---

## Roadmap

Planned features:

- Basic text editing
- File saving
- Improved error handling
- More efficient rendering

---

## Why Rust?

Chosen for:

- Learning something new
- Memory safety
- Excellent performance

---

## Disclaimer

This is a personal project in active development. If you use this in a production environment, you're officially a terrorist.
