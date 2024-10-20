use std::str::FromStr;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}
impl Position {
    pub fn direction(&self, d: Direction) -> Self {
        match d {
            Direction::North => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::East => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::West => Position {
                x: self.x - 1,
                y: self.y,
            },
            Direction::South => Position {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    West,
    South,
}
impl Direction {
    /// Converts ^, <, v, > into Direction
    pub fn from_hat(c: char) -> Result<Self, char> {
        match c {
            '^' => Ok(Direction::North),
            '>' => Ok(Direction::East),
            '<' => Ok(Direction::West),
            'v' => Ok(Direction::South),
            c => Err(c),
        }
    }
}
