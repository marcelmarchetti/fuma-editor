use std::fmt::{Display, Formatter, Result};

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Right,
    Left,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let s = match self {
            Direction::Right => "Right",
            Direction::Left  => "Left",
        };
        write!(f, "{}", s)
    }
}


impl Direction {
    #[inline]
    pub fn step(self) -> isize {
        match self {
            Direction::Right => 1,
            Direction::Left  => -1,
        }
    }
}