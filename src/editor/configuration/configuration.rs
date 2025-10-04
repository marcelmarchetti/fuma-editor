use std::{fs, io};
use toml::Value;
use crate::{log_debug, log_error, log_info};
use crate::editor::configuration::bindings::{KeysConfiguration};
use crate::editor::configuration::colors::ColorConfiguration;
use crate::editor::configuration::debug::DebugConfiguration;
use crate::editor::configuration::editor::EditorConfiguration;

pub struct Configuration {
    pub bindings: KeysConfiguration,
    pub editor: EditorConfiguration,
    pub debug: DebugConfiguration,
    pub colors: ColorConfiguration,
}

impl Configuration {
    pub fn new(toml_file: &Value) -> io::Result<Self> {
        Ok(
            Self {
                bindings: KeysConfiguration::from_toml(toml_file)?,
                editor: EditorConfiguration::new(toml_file),
                debug: DebugConfiguration::new(toml_file),
                colors: ColorConfiguration::from_toml(toml_file),
            }
        )
    }
    pub fn default() -> Self {
        Self {
            bindings: KeysConfiguration::default(),
            editor: EditorConfiguration::default(),
            debug: DebugConfiguration::default(),
            colors: ColorConfiguration::default(),
        }
    }
    pub fn apply_configuration(&self) {
        self.editor.apply_config();
        self.debug.apply_config();

    }
}



pub fn load_config() -> io::Result<Configuration> {
    let conf_content = fs::read_to_string("config.toml");

    match conf_content {
        Ok(content) => {
            let toml_file: Value = match toml::from_str(&content) {
                Ok(val) => { 
                    log_info!("Loaded configuration from TOML");    
                    val
                },
                Err(e) => {
                    log_error!("Invalid config file ({}), using default keybinds", e);
                    return Ok(Configuration::default());
                }
            };

            match Configuration::new(&toml_file) {
                Ok(cfg) => Ok(cfg),
                Err(_) => Ok(Configuration::default()),
            }
        }

        Err(_) => {
            log_error!("config.toml not found, using default keybinds");
            Ok(Configuration::default())
        }
    }
}

#[allow(dead_code)]
pub fn test_config() -> io::Result<()> {
    log_debug!("Testing key binding...");

    match load_config() {
        Ok(config) => {
            log_debug!("Configuration loaded!");
            log_debug!("Exit key: {:?}", config.bindings.quit);
            log_debug!("Move up key: {:?}", config.bindings.move_up);
            Ok(())
        },
        Err(e) => {
            log_error!("Error loading configuration: {}", e);
            Err(e)
        }
    }
}