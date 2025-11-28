use std::{fs, io};
use dirs::config_dir;
use toml::Value;
use crate::{log_debug, log_error, log_info};
use crate::editor::configuration::bindings::{KeysConfiguration};
use crate::editor::configuration::colors::ColorConfiguration;
use crate::editor::configuration::debug::DebugConfiguration;
use crate::editor::configuration::editor::EditorConfiguration;
use crate::values::globals::DEFAULT_CONFIG;

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
    let Some(mut path) = config_dir() else {
        log_error!("Could not determine config directory, using defaults");
        return Ok(Configuration::default());
    };

    path.push("fuma-editor");
    fs::create_dir_all(&path).ok();
    path.push("config.toml");

    if !path.exists() {
        log_info!("Config not found, creating default at {}", path.display());
        fs::write(&path, DEFAULT_CONFIG)?;
    }

    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => {
            log_error!("Could not read config, using defaults");
            return Ok(Configuration::default());
        }
    };

    match toml::from_str::<Value>(&content) {
        Ok(toml_file) => {
            match Configuration::new(&toml_file) {
                Ok(cfg) => {
                    log_info!("Loaded configuration from {}", path.display());
                    Ok(cfg)
                }
                Err(_) => {
                    log_error!("Config content invalid, using defaults (file kept as is)");
                    Ok(Configuration::default())
                }
            }
        }
        Err(e) => {
            log_error!("Invalid TOML in {}: {}", path.display(), e);
            log_error!("Using default config in memory (file NOT overwritten)");
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