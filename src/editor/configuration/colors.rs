use std::io;
use crossterm::style::{Color};
use serde::Deserialize;
use toml::Value;
use crate::log_error;
use crate::values::colors::{BASE, BLUE, CRUST, FLAMINGO, GREEN, LAVENDER, MANTLE, MAROON, MAUVE, OVERLAY0, OVERLAY1, OVERLAY2, PEACH, PINK, RED, ROSEWATER, SAPPHIRE, SKY, SUBTEXT0, SUBTEXT1, SURFACE0, SURFACE1, SURFACE2, TEAL, TEXT, YELLOW};

#[derive(Debug)]
pub struct ColorConfiguration {
    pub text_color: Color,
    pub line_numbering_color: Color,
    pub background_color: Color,
    pub dialog_color: Color,
    pub dialog_text_color: Color,
}
impl ColorConfiguration {
    pub fn default() -> Self {
        Self {
            text_color: TEXT,
            line_numbering_color: PEACH,
            background_color: BASE,
            dialog_color: SUBTEXT1,
            dialog_text_color: TEXT,

        }
    }
    pub fn from_toml(toml_file: &Value) -> Self {
        let raw = RawColors::new(toml_file);
        Self::from_raw(raw)
    }

    pub fn from_raw(raw: RawColors) -> Self {
        let default= ColorConfiguration::default();

        Self {
            text_color:  RawColors::str_to_color(&raw.text_color).unwrap_or(default.text_color),
            line_numbering_color:  RawColors::str_to_color(&raw.line_numbering_color).unwrap_or(default.line_numbering_color),
            background_color:  RawColors::str_to_color(&raw.background_color).unwrap_or(default.background_color),
            dialog_color:  RawColors::str_to_color(&raw.dialog_color).unwrap_or(default.dialog_color),
            dialog_text_color:  RawColors::str_to_color(&raw.dialog_text_color).unwrap_or(default.dialog_text_color),
        }
    }


}

#[derive(Debug, Deserialize)]
pub struct RawColors {
    pub text_color: String,
    pub line_numbering_color: String,
    pub background_color: String,
    pub dialog_color: String,
    pub dialog_text_color: String,
}

impl RawColors {
    pub fn new(toml_file: &Value) -> Self {
        match toml_file.get("color") {
            Some(debug_val) => debug_val.clone().try_into::<RawColors>().unwrap_or_else(|_| {
                log_error!("Invalid [color] section, using defaults");
                RawColors::default()
            }),
            None => RawColors::default()
        }
    }


    pub fn default() -> Self {
        Self {
            text_color: "text".to_string(),
            line_numbering_color: "peach".to_string(),
            background_color: "base".to_string(),
            dialog_color: "overlay0".to_string(),
            dialog_text_color: "subtext1".to_string()
        }
    }

    pub fn str_to_color(color: &str) -> io::Result<Color> {
        match color {
            "lavender" => Ok(LAVENDER),
            "text" => Ok(TEXT),
            "subtext1" => Ok(SUBTEXT1),
            "subtext0" => Ok(SUBTEXT0),
            "overlay2" => Ok(OVERLAY2),
            "overlay1" => Ok(OVERLAY1),
            "overlay0" => Ok(OVERLAY0),
            "surface2" => Ok(SURFACE2),
            "surface1" => Ok(SURFACE1),
            "surface0" => Ok(SURFACE0),
            "base" => Ok(BASE),
            "mantle" => Ok(MANTLE),
            "crust" => Ok(CRUST),
            "rosewater" => Ok(ROSEWATER),
            "flamingo" => Ok(FLAMINGO),
            "pink" => Ok(PINK),
            "mauve" => Ok(MAUVE),
            "red" => Ok(RED),
            "maroon" => Ok(MAROON),
            "peach" => Ok(PEACH),
            "yellow" => Ok(YELLOW),
            "green" => Ok(GREEN),
            "teal" => Ok(TEAL),
            "sky" => Ok(SKY),
            "sapphire" => Ok(SAPPHIRE),
            "blue" => Ok(BLUE),
            _ => Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Color '{}' in config.toml not found!", color),
            )),
        }
    }
}
