use std::ops::{Add, Rem};

///
/// Modulo that handles negative numbers, works the same as Python's `%`.
///
/// eg: `(a + b).modulo(c)`
///
/// returns always a non-negative number
pub trait SignedExt {
    #[must_use]
    fn modulo(&self, n: Self) -> Self;
}

impl<T> SignedExt for T
where
    Self: Copy,
    T: Rem<Output = T> + Add<Output = T>,
{
    fn modulo(&self, n: Self) -> Self {
        (*self % n + n) % n
    }
}
