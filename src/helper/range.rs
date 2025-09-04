use core::{
    cmp::Ord,
    ops::{Add, Range, RangeInclusive, Sub},
};

/// Trait to normalize access to start/end of ranges
pub trait Normalized<T> {
    fn start_bound(&self) -> T;
    fn end_bound(&self) -> T;
    fn is_inclusive(&self) -> bool;
}

impl<T: Copy> Normalized<T> for Range<T> {
    fn start_bound(&self) -> T {
        self.start
    }

    fn end_bound(&self) -> T {
        self.end
    }

    fn is_inclusive(&self) -> bool {
        false
    }
}

impl<T: Copy> Normalized<T> for RangeInclusive<T> {
    fn start_bound(&self) -> T {
        *self.start()
    }

    fn end_bound(&self) -> T {
        *self.end()
    }

    fn is_inclusive(&self) -> bool {
        true
    }
}

/// Extension trait for set-like operations on ranges
pub trait ExtRangeOps<T>: Normalized<T>
where
    T: Copy + Ord + Add<Output = T> + Sub<Output = T> + From<u8>,
    Self: Sized,
{
    fn intersection(&self, other: &Self) -> Option<Self>;
    fn union(&self, other: &Self) -> Option<Self>;
    fn overlaps(&self, other: &Self) -> bool;
    fn contains_range(&self, other: &Self) -> bool;
    fn subtract(&self, other: &Self) -> Vec<Self>;
}

// --- Implementations ---

impl<T> ExtRangeOps<T> for Range<T>
where
    T: Copy + Ord + Add<Output = T> + Sub<Output = T> + From<u8>,
{
    fn intersection(&self, other: &Self) -> Option<Self> {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);
        (start < end).then_some(start..end)
    }

    fn union(&self, other: &Self) -> Option<Self> {
        (self.overlaps(other) || self.end == other.start || other.end == self.start)
            .then(|| self.start.min(other.start)..self.end.max(other.end))
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.start < other.end && other.start < self.end
    }

    fn contains_range(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn subtract(&self, other: &Self) -> Vec<Self> {
        if !self.overlaps(other) {
            return vec![self.clone()];
        }

        let mut result = Vec::new();

        if self.start < other.start {
            result.push(self.start..other.start);
        }

        if self.end > other.end {
            result.push(other.end..self.end);
        }

        result
    }
}

impl<T> ExtRangeOps<T> for RangeInclusive<T>
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

        if self.start() < other.start() {
            result.push(*self.start()..=*other.start() - T::from(1));
        }

        if self.end() > other.end() {
            result.push(*other.end() + T::from(1)..=*self.end());
        }

        result
    }
}
