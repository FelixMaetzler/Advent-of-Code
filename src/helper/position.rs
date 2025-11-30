use core::num::TryFromIntError;
use core::{
    fmt::Debug,
    ops::{Add, AddAssign, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
    str::FromStr,
};

use crate::helper::misc::One;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct Position<T> {
    pub x: T,
    pub y: T,
}
impl TryFrom<Position<isize>> for Position<usize> {
    type Error = TryFromIntError;

    fn try_from(value: Position<isize>) -> Result<Self, Self::Error> {
        Ok(Self {
            x: usize::try_from(value.x)?,
            y: usize::try_from(value.y)?,
        })
    }
}
impl TryFrom<Position<usize>> for Position<isize> {
    type Error = TryFromIntError;

    fn try_from(value: Position<usize>) -> Result<Self, Self::Error> {
        Ok(Self {
            x: isize::try_from(value.x)?,
            y: isize::try_from(value.y)?,
        })
    }
}
impl<T> Position<T>
where
    T: Clone,
{
    pub fn as_xy_tuple(&self) -> (T, T) {
        (self.x.clone(), self.y.clone())
    }
    pub fn as_yx_tuple(&self) -> (T, T) {
        (self.y.clone(), self.x.clone())
    }
}
impl<T> Position<T> {
    pub fn from_yx<E>(s: &str) -> Result<Self, E>
    where
        T: FromStr<Err = E>,
    {
        let (y, x) = s.split_once(',').unwrap();
        let y = y.parse()?;
        let x = x.parse()?;
        Ok(Self { x, y })
    }
    pub fn from_xy<E>(s: &str) -> Result<Self, E>
    where
        T: FromStr<Err = E>,
    {
        let (x, y) = s.split_once(',').unwrap();
        let y = y.parse()?;
        let x = x.parse()?;
        Ok(Self { x, y })
    }
    pub fn manhattan_distance(&self, other: &Self) -> T
    where
        T: Add<Output = T> + Sub<Output = T> + Copy + Ord,
    {
        let x_diff = if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };

        let y_diff = if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        };

        x_diff + y_diff
    }
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
impl<T> SubAssign for Position<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
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
impl<T> AddAssign<T> for Position<T>
where
    T: AddAssign + Clone,
{
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs.clone();
        self.y += rhs;
    }
}
impl<T> AddAssign for Position<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl<T> MulAssign<T> for Position<T>
where
    T: MulAssign + Clone,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs.clone();
        self.y *= rhs;
    }
}
impl<T> DivAssign<T> for Position<T>
where
    T: DivAssign + Clone,
{
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs.clone();
        self.y /= rhs;
    }
}
impl<T> RemAssign for Position<T>
where
    T: RemAssign,
{
    fn rem_assign(&mut self, rhs: Self) {
        self.x %= rhs.x;
        self.y %= rhs.y;
    }
}
impl<T> Rem for Position<T>
where
    T: Rem<Output = T>,
{
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
        }
    }
}
impl<T> Mul<T> for Position<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> Position<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy + One,
{
    #[must_use]
    pub fn direction(&self, d: Direction4) -> Self {
        match d {
            Direction4::North => Self {
                x: self.x,
                y: self.y + T::one(),
            },
            Direction4::East => Self {
                x: self.x + T::one(),
                y: self.y,
            },
            Direction4::West => Self {
                x: self.x - T::one(),
                y: self.y,
            },
            Direction4::South => Self {
                x: self.x,
                y: self.y - T::one(),
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
    /// x,y.
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
impl From<(usize, usize)> for Position<usize> {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.1,
            y: value.0,
        }
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
    /// Converts ^, <, v, > into Direction.
    pub const fn from_hat(c: char) -> Result<Self, char> {
        match c {
            '^' => Ok(Self::North),
            '>' => Ok(Self::East),
            '<' => Ok(Self::West),
            'v' => Ok(Self::South),
            c => Err(c),
        }
    }
    /// Converts Direction into ^, <, v, >.
    pub const fn to_hat(self) -> char {
        match self {
            Self::North => '^',
            Self::East => '>',
            Self::West => '<',
            Self::South => 'v',
        }
    }
    #[must_use]
    pub const fn turn_right(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::West => Self::North,
            Self::South => Self::West,
        }
    }
    #[must_use]
    pub const fn turn_left(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::West => Self::South,
            Self::South => Self::East,
        }
    }
    #[must_use]
    pub const fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::West => Self::East,
            Self::South => Self::North,
        }
    }
    pub const fn all_dirs() -> [Self; 4] {
        use Direction4::{East, North, South, West};
        [North, East, South, West]
    }
}
impl From<Direction4> for Direction8 {
    fn from(val: Direction4) -> Self {
        match val {
            Direction4::North => Self::North,
            Direction4::East => Self::East,
            Direction4::West => Self::West,
            Direction4::South => Self::South,
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
    pub const fn all_dirs() -> [Self; 8] {
        use Direction8::{East, North, NorthEast, NorthWest, South, SouthEast, SouthWest, West};
        [
            North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest,
        ]
    }
    pub fn from_short_name(name: &str) -> Result<Self, &str> {
        match name {
            "n" => Ok(Self::North),
            "ne" => Ok(Self::NorthEast),
            "e" => Ok(Self::East),
            "se" => Ok(Self::SouthEast),
            "s" => Ok(Self::South),
            "sw" => Ok(Self::SouthWest),
            "w" => Ok(Self::West),
            "nw" => Ok(Self::NorthWest),
            x => Err(x),
        }
    }
}
