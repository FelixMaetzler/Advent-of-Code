use std::ops::{Add, AddAssign, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position3d<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Sub for Position3d<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl<T> SubAssign for Position3d<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl<T> Add for Position3d<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl<T> AddAssign<T> for Position3d<T>
where
    T: AddAssign + Clone,
{
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs.clone();
        self.y += rhs.clone();
        self.z += rhs;
    }
}
impl<T> AddAssign for Position3d<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl<T> MulAssign<T> for Position3d<T>
where
    T: MulAssign + Clone,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs.clone();
        self.y *= rhs.clone();
        self.z *= rhs;
    }
}
impl<T> DivAssign<T> for Position3d<T>
where
    T: DivAssign + Clone,
{
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs.clone();
        self.y /= rhs.clone();
        self.z /= rhs;
    }
}
impl<T> RemAssign for Position3d<T>
where
    T: RemAssign,
{
    fn rem_assign(&mut self, rhs: Self) {
        self.x %= rhs.x;
        self.y %= rhs.y;
        self.z %= rhs.z;
    }
}
impl<T> Rem for Position3d<T>
where
    T: Rem<Output = T>,
{
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
            z: self.z % rhs.z,
        }
    }
}
impl<T> Mul<T> for Position3d<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Position3d<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

macro_rules! impl_mul_for_type {
    ($type:ty) => {
        impl Mul<Position3d<$type>> for $type {
            type Output = Position3d<$type>;

            fn mul(self, rhs: Position3d<$type>) -> Self::Output {
                Position3d {
                    x: self * rhs.x,
                    y: self * rhs.y,
                    z: self * rhs.z,
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
