use std::hash::Hash;

use crate::helper::position::{Direction8, Position};

use super::Grid;

/// GridIndex is always a valid index into the Grid
pub trait GridIndex<T>
where
    Self: Copy + Clone + Eq + PartialEq + Hash,
{
    fn to_flat_index(&self, grid: &impl Grid<T>) -> usize;
    fn to_coordinates(&self, grid: &impl Grid<T>) -> (usize, usize);
    fn to_position(&self, grid: &impl Grid<T>) -> Position<usize>;
    fn dir(&self, dir: Direction8, grid: &impl Grid<T>) -> Option<(usize, usize)> {
        grid.get_dir8(*self, dir).map(|(x, _)| x)
    }
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
    #[inline(always)]
    fn to_position(&self, grid: &impl Grid<T>) -> Position<usize> {
        Position {
            x: self % grid.width(),
            y: self / grid.width(),
        }
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

    fn to_position(&self, _: &impl Grid<T>) -> Position<usize> {
        Position {
            x: self.1,
            y: self.0,
        }
    }
}
impl<T> GridIndex<T> for Position<usize> {
    #[inline(always)]
    fn to_flat_index(&self, grid: &impl Grid<T>) -> usize {
        debug_assert!(self.y < grid.height());
        debug_assert!(self.x < grid.width());
        self.y * grid.width() + self.x
    }
    #[inline(always)]
    fn to_coordinates(&self, _grid: &impl Grid<T>) -> (usize, usize) {
        self.as_yx_tuple()
    }

    fn to_position(&self, _: &impl Grid<T>) -> Position<usize> {
        *self
    }
}
