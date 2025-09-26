use std::io;
use crossterm::event::{KeyCode, KeyModifiers};
use serde::Deserialize;
use toml::Value;
use crate::log_error;
#[derive(Debug)]
pub struct KeyBind {
    pub main_key: KeyCode,
    pub modifier_key: KeyModifiers,
}

impl KeyBind {
    pub fn new(main_key: KeyCode, modifier_key: KeyModifiers) -> Self {
        Self { main_key, modifier_key }
    }

    pub fn from_raw(raw: &str) -> io::Result<Self> {
        let normalized = raw.to_lowercase();
        let parts: Vec<&str> = normalized.split_whitespace().collect();

        let mut main_key = KeyCode::Null;
        let mut modifier_key = KeyModifiers::NONE;

        for key in parts {
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
                        log_error!("Too many characters in '{}'", raw);
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Too many characters in '{}'", raw)));
                    }
                }
            }
        }

        if main_key == KeyCode::Null {
            log_error!("No main key in '{}'", raw);
            return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("No main key in '{}'", raw)));
        }

        Ok(KeyBind::new(main_key, modifier_key))
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

impl KeysConfiguration {
    pub fn from_raw(raw: RawKeysConfiguration) -> Self {
        Self {
            quit: KeyBind::from_raw(&raw.quit).unwrap_or_else(|_| KeysConfiguration::default().quit),
            move_up: KeyBind::from_raw(&raw.move_up).unwrap_or_else(|_| KeysConfiguration::default().move_up),
            move_down: KeyBind::from_raw(&raw.move_down).unwrap_or_else(|_| KeysConfiguration::default().move_down),
            move_left: KeyBind::from_raw(&raw.move_left).unwrap_or_else(|_| KeysConfiguration::default().move_left),
            move_right: KeyBind::from_raw(&raw.move_right).unwrap_or_else(|_| KeysConfiguration::default().move_right),
            move_to_start: KeyBind::from_raw(&raw.move_to_start).unwrap_or_else(|_| KeysConfiguration::default().move_to_start),
            move_to_end: KeyBind::from_raw(&raw.move_to_end).unwrap_or_else(|_| KeysConfiguration::default().move_to_end),
            move_token_left: KeyBind::from_raw(&raw.move_token_left).unwrap_or_else(|_| KeysConfiguration::default().move_token_left),
            move_token_right: KeyBind::from_raw(&raw.move_token_right).unwrap_or_else(|_| KeysConfiguration::default().move_token_right),
            get_token: KeyBind::from_raw(&raw.get_token).unwrap_or_else(|_| KeysConfiguration::default().get_token),
            tokenize_text: KeyBind::from_raw(&raw.tokenize_text).unwrap_or_else(|_| KeysConfiguration::default().tokenize_text),
            move_start_line: KeyBind::from_raw(&raw.move_start_line).unwrap_or_else(|_| KeysConfiguration::default().move_start_line),
            move_end_line: KeyBind::from_raw(&raw.move_end_line).unwrap_or_else(|_| KeysConfiguration::default().move_end_line),
        }
    }
    pub fn from_toml(toml_file: &Value) -> io::Result<Self> {
        let raw = RawKeysConfiguration::new(toml_file)?;
        Ok(Self::from_raw(raw))
    }
    pub(crate) fn default() -> Self {
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

#[derive(Debug, Deserialize)]
pub struct RawKeysConfiguration {
    pub quit: String,
    pub move_up: String,
    pub move_down: String,
    pub move_left: String,
    pub move_right: String,
    pub move_to_start: String,
    pub move_to_end: String,
    pub move_token_left: String,
    pub move_token_right: String,
    pub get_token: String,
    pub tokenize_text: String,
    pub move_start_line: String,
    pub move_end_line: String,
}

impl RawKeysConfiguration {
    pub fn new(toml_file: &Value) -> io::Result<Self> {
        let section = toml_file.get("bindings").expect("Missing [bindings] section]");
        let section_str = toml::to_string(section).unwrap();
        let raw: RawKeysConfiguration = toml::from_str(&section_str).expect("Failed to parse RawKeysConfiguration");
        Ok(raw)
    }
}


