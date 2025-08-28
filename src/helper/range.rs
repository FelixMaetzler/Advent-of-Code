use core::{
    cmp::Ord,
    ops::{Add, RangeInclusive, Sub},
};

/// Extension trait for `RangeInclusive<T>` to provide set-like operations for integer types.
pub trait Ext<T>
where
    T: Copy + Ord + Add<Output = T> + Sub<Output = T> + From<u8>,
{
    fn intersection(&self, other: &Self) -> Option<RangeInclusive<T>>;
    fn union(&self, other: &Self) -> Option<RangeInclusive<T>>;
    fn overlaps(&self, other: &Self) -> bool;
    fn contains_range(&self, other: &Self) -> bool;
    fn subtract(&self, other: &Self) -> Vec<RangeInclusive<T>>;
}

impl<T> Ext<T> for RangeInclusive<T>
where
    T: Copy + Ord + Add<Output = T> + Sub<Output = T> + From<u8>,
{
    fn intersection(&self, other: &Self) -> Option<Self> {
        let start = *self.start().max(other.start());
        let end = *self.end().min(other.end());
        (start <= end).then_some(start..=end)
    }

    fn union(&self, other: &Self) -> Option<Self> {
        (self.overlaps(other)
            || *self.end() + T::from(1) == *other.start()
            || *other.end() + T::from(1) == *self.start())
        .then(|| *self.start().min(other.start())..=*self.end().max(other.end()))
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.start() <= other.end() && other.start() <= self.end()
    }

    fn contains_range(&self, other: &Self) -> bool {
        self.start() <= other.start() && self.end() >= other.end()
    }

    fn subtract(&self, other: &Self) -> Vec<Self> {
        if !self.overlaps(other) {
            return vec![self.clone()];
        }

        let mut result = Vec::new();
        let start = *self.start();
        let end = *self.end();

        // Left segment
        if start < *other.start() {
            result.push(start..=other.start().sub(T::from(1)));
        }

        // Right segment
        if end > *other.end() {
            result.push(other.end().add(T::from(1))..=end);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_subtract() {
        assert_eq!(
            (1_u32..=10_u32).subtract(&(3_u32..=7_u32)),
            vec![1_u32..=2_u32, 8_u32..=10_u32]
        );
        assert_eq!(
            (1_u32..=5_u32).subtract(&(6_u32..=10_u32)),
            vec![1_u32..=5_u32]
        );
        assert_eq!((1_u32..=5_u32).subtract(&(1_u32..=5_u32)), vec![]);
        assert_eq!(
            (1_u32..=5_u32).subtract(&(3_u32..=10_u32)),
            vec![1_u32..=2_u32]
        );
        assert_eq!(
            (3_u32..=10_u32).subtract(&(1_u32..=5_u32)),
            vec![6_u32..=10_u32]
        );
    }

    #[test]
    fn test_i32_intersection() {
        assert_eq!(
            (-5_i32..=5_i32).intersection(&(0_i32..=10_i32)),
            Some(0_i32..=5_i32)
        );
        assert_eq!((-5_i32..=-1_i32).intersection(&(0_i32..=10_i32)), None);
    }
}
