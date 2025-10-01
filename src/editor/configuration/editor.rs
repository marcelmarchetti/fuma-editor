use std::sync::atomic::Ordering;
use serde::Deserialize;
use toml::Value;
use crate::log_error;
use crate::values::globals::SHOW_LINE_NUMBERING;

#[derive(Debug, Deserialize)]
pub struct EditorConfiguration {
    pub line_numbering: bool
}

impl EditorConfiguration {
    pub fn default() -> Self {
        Self {
            line_numbering: false
        }
    }
}

impl EditorConfiguration {
    pub fn new(toml_file: &Value) -> Self {
        match toml_file.get("editor") {
            Some(editor_val) => editor_val.clone().try_into::<EditorConfiguration>().unwrap_or_else(|_| {
                log_error!("Invalid [editor] section, using defaults");
                EditorConfiguration::default()
            }),
            None => EditorConfiguration::default()
        }
    }
    
    pub fn apply_config(&self) {
        SHOW_LINE_NUMBERING.store(self.line_numbering, Ordering::Relaxed);
    }
}