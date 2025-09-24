use std::{fs, io};
use crossterm::event::{KeyCode, KeyModifiers};
use crate::{log_debug, log_error};

#[derive(Debug)]
pub struct KeyBind{
    pub main_key: KeyCode,
    pub modifier_key: KeyModifiers,
}

impl KeyBind{
    pub fn new(main_key: KeyCode, modifier_key: KeyModifiers) -> Self{
        Self {
            main_key,
            modifier_key
        }
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
    fn new(json_file: serde_json::Value) -> io::Result<Self> {
        Ok(Self {
            quit: parse_to_keybind("quit", &json_file)?,
            move_up: parse_to_keybind("move_up", &json_file)?,
            move_down: parse_to_keybind("move_down", &json_file)?,
            move_left: parse_to_keybind("move_left", &json_file)?,
            move_right: parse_to_keybind("move_right", &json_file)?,
            move_to_start: parse_to_keybind("move_to_start", &json_file)?,
            move_to_end: parse_to_keybind("move_to_end", &json_file)?,
            move_token_left: parse_to_keybind("move_token_left", &json_file)?,
            move_token_right: parse_to_keybind("move_token_right", &json_file)?,
            get_token: parse_to_keybind("get_token", &json_file)?,
            tokenize_text: parse_to_keybind("tokenize_text", &json_file)?,
            move_start_line: parse_to_keybind("move_start_line", &json_file)?,
            move_end_line: parse_to_keybind("move_end_line", &json_file)?,
        })
    }
}


pub fn load_config() -> io::Result<KeysConfiguration> {
    let conf_file = fs::File::open("config.json");

    match conf_file {
        Ok(file) => {
            let json: serde_json::Value = match serde_json::from_reader(file) {
                Ok(val) => val,
                Err(e) => {
                    log_error!("Invalid config file ({}), using default keybinds", e);
                    return Ok(KeysConfiguration::default());
                }
            };

            match KeysConfiguration::new(json) {
                Ok(cfg) => Ok(cfg),
                Err(e) => {
                    log_error!("Invalid config file ({}), using default keybinds", e);
                    Ok(KeysConfiguration::default())
                }
            }
        }
        Err(_) => {
            log_error!("config.json not found , using default keybinds");
            Ok(KeysConfiguration::default())
        }
    }
}


fn parse_to_keybind (instruction_key: &str, config_file: &serde_json::Value) -> io::Result<KeyBind>  {
    let raw_bind = config_file[instruction_key].as_str().ok_or_else(||
        {
            log_error!("Invalid config file ({}), using default keybinds", instruction_key);
            io::Error::new(io::ErrorKind::InvalidInput, format!("key '{}' not found or not a string in config", instruction_key))
        })?.to_string();

    let normalized_raw_bind =  raw_bind.to_lowercase();
    let binds:Vec<&str> = normalized_raw_bind.split(' ').collect();

    let mut main_key: KeyCode = KeyCode::Null;
    let mut modifier_key = KeyModifiers::NONE;

    if binds.len() > 3 {
        log_error!("too many arguments on '{}' key bind", raw_bind);
        return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("too many arguments on '{}' key bind", raw_bind)));
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
                    main_key = KeyCode::Char(key.chars().next().ok_or_else(|| {
                        log_error!("Invalid character in '{}'", key);
                        io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid character in '{}'", key),
                        )
                    })?
                    );
                } else {
                    log_error!("Too many characters on '{}' in '{}'", main_key, raw_bind);
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Too many characters on '{}' in '{}'", main_key, raw_bind)));
                }
            }
        }
    }

    if main_key == KeyCode::Null {
        log_error!("no main key in '{}'", raw_bind);
        return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("no main key in '{}'", raw_bind)))
    }

    Ok(KeyBind::new(
        main_key,
        modifier_key
    ))
}

#[allow(dead_code)]
pub fn test_config() -> io::Result<()> {
    log_debug!("Testing key binding...");

    match load_config() {
        Ok(config) => {
            log_debug!("Configuration loaded!");
            log_debug!("Exit key: {:?}", config.quit);
            log_debug!("Move up key: {:?}", config.move_up);
            Ok(())
        },
        Err(e) => {
            log_error!("Error loading configuration: {}", e);
            Err(e)
        }
    }
}