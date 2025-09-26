use std::{fs, io};
use toml::Value;
use crate::{log_debug, log_error};
use crate::editor::configuration::bindings::KeysConfiguration;

pub fn load_config() -> io::Result<> {
    let conf_content = fs::read_to_string("config.toml");

    match conf_content {
        Ok(content) => {
            let toml_file: Value = match toml::from_str(&content) {
                Ok(val) => val,
                Err(e) => {
                    log_error!("Invalid config file ({}), using default keybinds", e);
                    return Ok(KeysConfiguration::default());
                }
            };

            match KeysConfiguration::new(&toml_file) {
                Ok(cfg) => Ok(cfg),
                Err(_) => Ok(KeysConfiguration::default()),
            }
        }

        Err(_) => {
            log_error!("config.toml not found, using default keybinds");
            Ok(KeysConfiguration::default())
        }
    }
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
