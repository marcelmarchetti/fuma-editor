#[derive(Copy, Clone)]
pub enum Direction {
    Right,
    Left,
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