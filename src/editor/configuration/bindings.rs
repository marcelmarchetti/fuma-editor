use std::{fs, io};
use crossterm::event::{KeyCode, KeyModifiers};
use crate::{log_debug, log_error};
use toml::Value;

#[derive(Debug)]
pub struct KeyBind {
    pub main_key: KeyCode,
    pub modifier_key: KeyModifiers,
}

impl KeyBind {
    pub fn new(main_key: KeyCode, modifier_key: KeyModifiers) -> Self {
        Self { main_key, modifier_key }
    }
}

pub struct KeysConfiguration {
    pub quit: KeyBind,
    pub move_up: KeyBind,
    pub move_down: KeyBind,
    pub move_left: KeyBind,
    pub move_right: KeyBind,
    pub move_to_start: KeyBind,
    pub move_to_end: KeyBind,
    pub move_token_left: KeyBind,
    pub move_token_right: KeyBind,
    pub get_token: KeyBind,
    pub tokenize_text: KeyBind,
    pub move_start_line: KeyBind,
    pub move_end_line: KeyBind,
}

impl Default for KeysConfiguration {
    fn default() -> Self {
        Self {
            quit: KeyBind::new(KeyCode::Char('q'), KeyModifiers::CONTROL),
            move_up: KeyBind::new(KeyCode::Up, KeyModifiers::NONE),
            move_down: KeyBind::new(KeyCode::Down, KeyModifiers::NONE),
            move_left: KeyBind::new(KeyCode::Left, KeyModifiers::NONE),
            move_right: KeyBind::new(KeyCode::Right, KeyModifiers::NONE),
            move_to_start: KeyBind::new(KeyCode::Home, KeyModifiers::NONE),
            move_to_end: KeyBind::new(KeyCode::End, KeyModifiers::NONE),
            move_token_left: KeyBind::new(KeyCode::Left, KeyModifiers::CONTROL),
            move_token_right: KeyBind::new(KeyCode::Right, KeyModifiers::CONTROL),
            get_token: KeyBind::new(KeyCode::Char('t'), KeyModifiers::CONTROL),
            tokenize_text: KeyBind::new(KeyCode::Char('t'), KeyModifiers::NONE),
            move_start_line: KeyBind::new(KeyCode::Char('h'), KeyModifiers::CONTROL),
            move_end_line: KeyBind::new(KeyCode::Char('l'), KeyModifiers::CONTROL),
        }
    }
}

impl KeysConfiguration {
    pub(crate) fn new(toml_file: &Value) -> io::Result<Self> {
        Ok(Self {
            quit: parse_to_keybind("quit", toml_file)?,
            move_up: parse_to_keybind("move_up", toml_file)?,
            move_down: parse_to_keybind("move_down", toml_file)?,
            move_left: parse_to_keybind("move_left", toml_file)?,
            move_right: parse_to_keybind("move_right", toml_file)?,
            move_to_start: parse_to_keybind("move_to_start", toml_file)?,
            move_to_end: parse_to_keybind("move_to_end", toml_file)?,
            move_token_left: parse_to_keybind("move_token_left", toml_file)?,
            move_token_right: parse_to_keybind("move_token_right", toml_file)?,
            get_token: parse_to_keybind("get_token", toml_file)?,
            tokenize_text: parse_to_keybind("tokenize_text", toml_file)?,
            move_start_line: parse_to_keybind("move_start_line", toml_file)?,
            move_end_line: parse_to_keybind("move_end_line", toml_file)?,
        })
    }
}

fn parse_to_keybind(instruction_key: &str, config_file: &Value) -> io::Result<KeyBind> {
    let bindings = config_file
        .get("bindings")
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing [bindings] section"))?;

    let raw_bind = bindings[instruction_key]
        .as_str()
        .ok_or_else(|| {
            log_error!("Invalid config file ({}), using default keybinds", instruction_key);
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("key '{}' not found or not a string", instruction_key),
            )
        })?
        .to_string();

    let normalized_raw_bind = raw_bind.to_lowercase();
    let binds: Vec<&str> = normalized_raw_bind.split_whitespace().collect();

    let mut main_key: KeyCode = KeyCode::Null;
    let mut modifier_key = KeyModifiers::NONE;

    if binds.len() > 3 {
        log_error!("too many arguments on '{}' key bind", raw_bind);
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("too many arguments on '{}' key bind", raw_bind),
        ));
    }

    for key in binds {
        match key {
            "control" => modifier_key = KeyModifiers::CONTROL,
            "shift" => modifier_key = KeyModifiers::SHIFT,
            "alt" => modifier_key = KeyModifiers::ALT,
            "up" => main_key = KeyCode::Up,
            "down" => main_key = KeyCode::Down,
            "left" => main_key = KeyCode::Left,
            "right" => main_key = KeyCode::Right,
            "home" => main_key = KeyCode::Home,
            "end" => main_key = KeyCode::End,
            _ => {
                if key.chars().count() == 1 {
                    main_key = KeyCode::Char(key.chars().next().unwrap());
                } else {
                    log_error!("Too many characters on '{}' in '{}'", main_key, raw_bind);
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("Too many characters on '{}' in '{}'", main_key, raw_bind),
                    ));
                }
            }
        }
    }

    if main_key == KeyCode::Null {
        log_error!("no main key in '{}'", raw_bind);
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("no main key in '{}'", raw_bind),
        ));
    }

    Ok(KeyBind::new(main_key, modifier_key))
}