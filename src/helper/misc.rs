use core::hash::Hash;
use core::ops::{AddAssign, Div, Mul, Rem, SubAssign};
use std::collections::HashMap;

pub fn number_to_digit_count(x: u64) -> u8 {
    x.checked_ilog10()
        .map_or(1, |x| u8::try_from(x + 1).unwrap())
}
pub trait Joinable<T> {
    fn join(self, separator: &str) -> String;
}

impl<I, T> Joinable<T> for I
where
    I: Iterator<Item = T>,
    T: ToString,
{
    fn join(self, separator: &str) -> String {
        let mut iter = self;
        let mut result = String::new();

        if let Some(first) = iter.next() {
            result.push_str(&first.to_string());
        }

        for item in iter {
            result.push_str(separator);
            result.push_str(&item.to_string());
        }

        result
    }
}
// Counter
pub fn count_occurrences<T, I>(iterable: I) -> HashMap<T, usize>
where
    T: Eq + Hash,
    I: IntoIterator<Item = T>,
{
    let mut counts = HashMap::new();
    for item in iterable {
        *counts.entry(item).or_insert(0) += 1;
    }
    counts
}
// Define the Zero trait
pub trait Zero {
    fn zero() -> Self;
}
// Implement Zero for different types using a macro
macro_rules! impl_zero_for_types {
    ($($t:ty),*) => {
        $(
            impl Zero for $t {
                #[allow(clippy::allow_attributes, reason = "doesnt lint for every type")]
                #[allow(clippy::cast_lossless, reason = "doesnt lint for every type")]
                #[allow(clippy::cast_precision_loss, reason = "doenst loose precision")]
                fn zero() -> Self {
                    // Return zero for each type
                    0 as $t
                }
            }
        )*
    };
}

impl_zero_for_types!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
); // Define the Zero trait
pub trait One {
    fn one() -> Self;
}

// Implement One for different types using a macro
macro_rules! impl_one_for_types {
    ($($t:ty),*) => {
        $(
            impl One for $t {
                #[allow(clippy::allow_attributes, reason = "doesnt lint for every type")]
                #[allow(clippy::cast_lossless, reason = "doesnt lint for every type")]
                #[allow(clippy::cast_precision_loss, reason = "doenst loose precision")]
                fn one() -> Self {
                    // Return one for each type
                    1 as $t
                }
            }
        )*
    };
}

impl_one_for_types!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
);

pub trait Unsigned {}

// Implement the trait for all unsigned types using a macro
macro_rules! impl_unsigned_trait_for_unsigned {
    ($($t:ty),*) => {
        $(
            impl Unsigned for $t {
            }
        )*
    };
}
impl_unsigned_trait_for_unsigned!(u8, u16, u32, u64, u128, usize);
pub fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: Copy + Ord + Rem<Output = T> + Zero + SubAssign + AddAssign + Unsigned,
{
    while b != T::zero() {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}
pub fn lcm<T>(a: T, b: T) -> T
where
    T: Copy
        + Ord
        + Unsigned
        + Rem<Output = T>
        + Zero
        + SubAssign
        + AddAssign
        + Mul<Output = T>
        + Div<Output = T>,
{
    a * b / gcd(a, b)
}

pub fn lcm3<T>(a: T, b: T, c: T) -> T
where
    T: Copy
        + Ord
        + Unsigned
        + Rem<Output = T>
        + Zero
        + SubAssign
        + AddAssign
        + Mul<Output = T>
        + Div<Output = T>,
{
    lcm(a, lcm(b, c))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_to_digit_count() {
        assert_eq!(number_to_digit_count(0), 1);
        assert_eq!(number_to_digit_count(1), 1);
        assert_eq!(number_to_digit_count(9), 1);
        assert_eq!(number_to_digit_count(10), 2);
        assert_eq!(number_to_digit_count(99), 2);
        assert_eq!(number_to_digit_count(100), 3);
    }
}
