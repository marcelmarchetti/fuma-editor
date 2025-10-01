use std::sync::atomic::Ordering;
use serde::Deserialize;
use toml::Value;
use crate::log_error;
use crate::values::globals::{DEBUG_SELECTION, DEBUG_TOKENIZER, DEBUG_WRAPPING};

#[derive(Debug, Deserialize)]
pub struct DebugConfiguration {
    pub debug_wrapping: bool,
    pub debug_tokenizer: bool,
    pub debug_selection: bool,
}

impl DebugConfiguration {
    pub fn default() -> Self {
        Self {
            debug_wrapping: false,
            debug_tokenizer: false,
            debug_selection: false,
        }
    }

    pub fn new(toml_file: &Value) -> Self {
        match toml_file.get("debug") {
            Some(debug_val) => debug_val.clone().try_into::<DebugConfiguration>().unwrap_or_else(|_| {
                log_error!("Invalid [debug] section, using defaults");
                DebugConfiguration::default()
            }),
            None => DebugConfiguration::default()
        }
    }
    pub fn apply_config(&self) {
        DEBUG_WRAPPING.store(self.debug_wrapping, Ordering::Relaxed);
        DEBUG_TOKENIZER.store(self.debug_tokenizer, Ordering::Relaxed);
        DEBUG_SELECTION.store(self.debug_selection, Ordering::Relaxed);
    }
}

