use std::fmt;
use std::fmt::Display;

#[derive(PartialEq)]
pub enum Separator {
    Comma,
    Colon,
    Semicolon,
}


impl Display for Separator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Separator::Comma => ",",
            Separator::Colon => ":",
            Separator::Semicolon => ";",
        };
        write!(f, "separator  {}", s)
    }
}

impl Separator {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "," => Some(Separator::Comma),
            ";" => Some(Separator::Semicolon),
            ":" => Some(Separator::Colon),
            _ => None,
        }
    }
}
