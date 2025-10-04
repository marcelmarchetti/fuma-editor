use std::fmt;

#[derive(PartialEq)]
pub enum Punctuation {
    PathSeparator,
    ReturnArrow,
    FatArrow,
}

impl Punctuation {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "::" => Some(Punctuation::PathSeparator),
            "->" => Some(Punctuation::ReturnArrow),
            "=>" => Some(Punctuation::FatArrow),
            _ => None,
        }
    }
}

impl fmt::Display for Punctuation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Punctuation::PathSeparator => write!(f, "::"),
            Punctuation::ReturnArrow => write!(f, "->"),
            Punctuation::FatArrow => write!(f, "=>"),
        }
    }
}
