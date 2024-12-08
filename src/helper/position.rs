use std::{
    fmt::Debug,
    ops::{Add, Mul, Sub},
    str::FromStr,
};

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position<T> {
    pub x: T,
    pub y: T,
}
impl<T> Sub for Position<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl<T> Add for Position<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl<T> Mul<T> for Position<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Position<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> Position<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy,
    i32: Into<T>,
{
    pub fn direction(&self, d: Direction4) -> Self {
        match d {
            Direction4::North => Position {
                x: self.x,
                y: self.y + 1.into(),
            },
            Direction4::East => Position {
                x: self.x + 1.into(),
                y: self.y,
            },
            Direction4::West => Position {
                x: self.x - 1.into(),
                y: self.y,
            },
            Direction4::South => Position {
                x: self.x,
                y: self.y - 1.into(),
            },
        }
    }
}
impl<T> FromStr for Position<T>
where
    T: FromStr,
    T::Err: Debug,
{
    type Err = ();
    /// x,y
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
        Ok(Self { x, y })
    }
}
macro_rules! impl_mul_for_type {
    ($type:ty) => {
        impl Mul<Position<$type>> for $type {
            type Output = Position<$type>;

            fn mul(self, rhs: Position<$type>) -> Self::Output {
                Position {
                    x: self * rhs.x,
                    y: self * rhs.y,
                }
            }
        }
    };
}
macro_rules! call_macro_with_types {
    ($macro_name:ident, [$($type:ty),*]) => {
        $(
            $macro_name!($type);
        )*
    };
}
call_macro_with_types!(
    impl_mul_for_type,
    [u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64]
);

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
