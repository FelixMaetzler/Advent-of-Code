use std::hash::Hash;

use super::Grid;

/// GridIndex is always a valid index into the Grid
pub trait GridIndex<T>
where
    Self: Copy + Eq + PartialEq + Hash,
{
    fn to_flat_index(&self, grid: &impl Grid<T>) -> usize;
    fn to_coordinates(&self, grid: &impl Grid<T>) -> (usize, usize);
}
impl<T> GridIndex<T> for usize {
    #[inline(always)]
    fn to_flat_index(&self, _grid: &impl Grid<T>) -> usize {
        *self
    }
    #[inline(always)]
    fn to_coordinates(&self, grid: &impl Grid<T>) -> (usize, usize) {
        (self / grid.width(), self % grid.width())
    }
}
impl<T> GridIndex<T> for (usize, usize) {
    #[inline(always)]
    fn to_flat_index(&self, grid: &impl Grid<T>) -> usize {
        debug_assert!(self.0 < grid.height());
        debug_assert!(self.1 < grid.width());
        self.0 * grid.width() + self.1
    }
    #[inline(always)]
    fn to_coordinates(&self, _grid: &impl Grid<T>) -> (usize, usize) {
        *self
    }
}
