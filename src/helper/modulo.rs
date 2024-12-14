use std::ops::{Add, Rem};

///
/// Modulo that handles negative numbers, works the same as Python's `%`.
///
/// eg: `(a + b).modulo(c)`
///
/// returns always a non-negative number
pub trait ModuloSignedExt {
    fn modulo(&self, n: Self) -> Self;
}

impl<T> ModuloSignedExt for T
where
    Self: Copy,
    T: Rem<Output = T> + Add<Output = T>,
{
    #[inline(always)]
    fn modulo(&self, n: Self) -> Self {
        (*self % n + n) % n
    }
}
