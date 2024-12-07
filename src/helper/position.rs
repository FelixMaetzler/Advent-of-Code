use std::str::FromStr;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}
impl Position {
    pub fn direction(&self, d: Direction4) -> Self {
        match d {
            Direction4::North => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction4::East => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction4::West => Position {
                x: self.x - 1,
                y: self.y,
            },
            Direction4::South => Position {
                x: self.x,
                y: self.y - 1,
            },
        }
    }
}
impl FromStr for Position {
    type Err = ();
    /// x,y
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
        Ok(Self { x, y })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction4 {
    North,
    East,
    West,
    South,
}
impl Direction4 {
    /// Converts ^, <, v, > into Direction
    pub fn from_hat(c: char) -> Result<Self, char> {
        match c {
            '^' => Ok(Direction4::North),
            '>' => Ok(Direction4::East),
            '<' => Ok(Direction4::West),
            'v' => Ok(Direction4::South),
            c => Err(c),
        }
    }

    pub fn turn_right(&self) -> Direction4 {
        match self {
            Direction4::North => Self::East,
            Direction4::East => Self::South,
            Direction4::West => Self::North,
            Direction4::South => Self::West,
        }
    }
}
impl From<Direction4> for Direction8 {
    fn from(val: Direction4) -> Self {
        match val {
            Direction4::North => Direction8::North,
            Direction4::East => Direction8::East,
            Direction4::West => Direction8::West,
            Direction4::South => Direction8::South,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction8 {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}
impl Direction8 {
    pub fn all_dirs() -> [Direction8; 8] {
        use Direction8::*;
        [
            North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest,
        ]
    }
}
