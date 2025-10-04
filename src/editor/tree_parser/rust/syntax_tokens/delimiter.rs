use std::fmt;
use std::fmt::Display;

#[derive(PartialEq)]
pub enum Delimiter {
    BraceL,
    BraceR,
    ParenL,
    ParenR,
    BracketL,
    BracketR,
}

impl Display for Delimiter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Delimiter::BraceL => "{",
            Delimiter::BraceR => "}",
            Delimiter::ParenL => "(",
            Delimiter::ParenR => ")",
            Delimiter::BracketL => "[",
            Delimiter::BracketR => "]",
        };
        write!(f, "delimiter  {}", s)
    }
}

impl Delimiter {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "{" => Some(Delimiter::BraceL),
            "}" => Some(Delimiter::BraceR),
            "(" => Some(Delimiter::ParenL),
            ")" => Some(Delimiter::ParenR),
            "[" => Some(Delimiter::BracketL),
            "]" => Some(Delimiter::BracketR),
            _ => None,
        }
    }
}

