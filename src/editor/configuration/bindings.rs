use std::io;
use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::event::Event::Key;
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

    pub fn from_raw(raw: &str, only_modifier: bool) -> io::Result<Self> {
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
                        let c = key.chars().next()
                            .ok_or_else(|| {
                                log_error!("No bind found for file '{}'", raw);
                                io::Error::new(io::ErrorKind::InvalidInput, format!("No bind found for file '{}'", raw))
                            })?;

                        main_key = KeyCode::Char(c);
                    }else {
                        log_error!("Too many characters in '{}'", raw);
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Too many characters in '{}'", raw)));
                    }
                }
            }
        }

        if main_key == KeyCode::Null && !only_modifier {
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
    //pub get_token: KeyBind,
    //pub tokenize_text: KeyBind,
    pub move_start_line: KeyBind,
    pub move_end_line: KeyBind,
    pub delete_line: KeyBind,
    pub save_file: KeyBind,
    pub copy: KeyBind,
    pub paste: KeyBind,
    pub cut: KeyBind,
    pub select_key: KeyBind,
}

impl KeysConfiguration {
    pub fn from_raw(raw: RawKeysConfiguration) -> Self {
        Self {
            quit: KeyBind::from_raw(&raw.quit, false).unwrap_or_else(|_| KeysConfiguration::default().quit),
            move_up: KeyBind::from_raw(&raw.move_up, false).unwrap_or_else(|_| KeysConfiguration::default().move_up),
            move_down: KeyBind::from_raw(&raw.move_down, false).unwrap_or_else(|_| KeysConfiguration::default().move_down),
            move_left: KeyBind::from_raw(&raw.move_left, false).unwrap_or_else(|_| KeysConfiguration::default().move_left),
            move_right: KeyBind::from_raw(&raw.move_right, false).unwrap_or_else(|_| KeysConfiguration::default().move_right),
            move_to_start: KeyBind::from_raw(&raw.move_to_start, false).unwrap_or_else(|_| KeysConfiguration::default().move_to_start),
            move_to_end: KeyBind::from_raw(&raw.move_to_end, false).unwrap_or_else(|_| KeysConfiguration::default().move_to_end),
            move_token_left: KeyBind::from_raw(&raw.move_token_left, false).unwrap_or_else(|_| KeysConfiguration::default().move_token_left),
            move_token_right: KeyBind::from_raw(&raw.move_token_right, false).unwrap_or_else(|_| KeysConfiguration::default().move_token_right),
            //get_token: KeyBind::from_raw(&raw.get_token).unwrap_or_else(|_| KeysConfiguration::default().get_token),
            //tokenize_text: KeyBind::from_raw(&raw.tokenize_text).unwrap_or_else(|_| KeysConfiguration::default().tokenize_text),
            move_start_line: KeyBind::from_raw(&raw.move_start_line, false).unwrap_or_else(|_| KeysConfiguration::default().move_start_line),
            move_end_line: KeyBind::from_raw(&raw.move_end_line, false).unwrap_or_else(|_| KeysConfiguration::default().move_end_line),
            delete_line: KeyBind::from_raw(&raw.delete_line, false).unwrap_or_else(|_| KeysConfiguration::default().delete_line),
            save_file: KeyBind::from_raw(&raw.save_file, false).unwrap_or_else(|_| KeysConfiguration::default().save_file),
            copy: KeyBind::from_raw(&raw.copy, false).unwrap_or_else(|_| KeysConfiguration::default().copy),
            paste: KeyBind::from_raw(&raw.paste, false).unwrap_or_else(|_| KeysConfiguration::default().paste),
            cut: KeyBind::from_raw(&raw.cut, false).unwrap_or_else(|_| KeysConfiguration::default().cut),
            select_key: KeyBind::from_raw(&raw.select_key, true).unwrap_or_else(|_| KeysConfiguration::default().select_key),
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
            //get_token: KeyBind::new(KeyCode::Char('t'), KeyModifiers::CONTROL),
            //tokenize_text: KeyBind::new(KeyCode::Char('t'), KeyModifiers::NONE),
            move_start_line: KeyBind::new(KeyCode::Char('h'), KeyModifiers::CONTROL),
            move_end_line: KeyBind::new(KeyCode::Char('l'), KeyModifiers::CONTROL),
            delete_line: KeyBind::new(KeyCode::Char('d'), KeyModifiers::CONTROL),
            save_file: KeyBind::new(KeyCode::Char('s'), KeyModifiers::CONTROL),
            copy: KeyBind::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
            paste: KeyBind::new(KeyCode::Char('v'), KeyModifiers::CONTROL),
            cut: KeyBind::new(KeyCode::Char('x'), KeyModifiers::CONTROL),
            select_key: KeyBind::new(KeyCode::Null, KeyModifiers::SHIFT),

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
    //pub get_token: String,
    //pub tokenize_text: String,
    pub move_start_line: String,
    pub move_end_line: String,
    pub delete_line: String,
    pub save_file: String,
    pub copy: String,
    pub paste: String,
    pub cut: String,
    pub select_key: String,
}

impl RawKeysConfiguration {
    pub fn new(toml_file: &Value) -> io::Result<Self> {
        let section = toml_file
            .get("bindings")
            .ok_or_else(|| {
                log_error!("Missing [bindings] section");
                io::Error::new(io::ErrorKind::InvalidData, "Missing [bindings] section") })?;

        let section_str = toml::to_string(section)
            .map_err(|e| {
                log_error!("Failed to serialize bindings: {}", e);
                io::Error::new(io::ErrorKind::InvalidData, format!("Failed to serialize bindings: {}", e)) })?;

        let raw: RawKeysConfiguration = toml::from_str(&section_str)
            .map_err(|e| {
                log_error!("Failed to parse RawKeysConfiguration: {}", e);
                io::Error::new(io::ErrorKind::InvalidData, format!("Failed to parse RawKeysConfiguration: {}", e)) })?;

        Ok(raw)
    }
}

