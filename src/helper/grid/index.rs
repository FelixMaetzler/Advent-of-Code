use core::hash::Hash;

use crate::helper::position::{Direction8, Position};

use super::Grid;

/// `GridIndex` is always a valid index into the Grid
#[expect(clippy::module_name_repetitions, reason = "makes more sense")]
pub trait GridIndex<T>
where
    Self: Copy + Clone + Eq + PartialEq + Hash,
{
    fn to_flat_index<G>(&self, grid: &G) -> usize
    where
        G: Grid<T>;
    fn to_coordinates<G>(&self, grid: &G) -> (usize, usize)
    where
        G: Grid<T>;
    fn to_position<G>(&self, grid: &G) -> Position<usize>
    where
        G: Grid<T>;
    fn dir<G>(&self, dir: Direction8, grid: &G) -> Option<(usize, usize)>
    where
        G: Grid<T>,
    {
        grid.get_dir8(*self, dir).map(|(x, _)| x)
    }
}
impl<T> GridIndex<T> for usize {
    fn to_flat_index<G>(&self, _grid: &G) -> usize
    where
        G: Grid<T>,
    {
        *self
    }

    fn to_coordinates<G>(&self, grid: &G) -> (usize, usize)
    where
        G: Grid<T>,
    {
        (self / grid.width(), self % grid.width())
    }

    fn to_position<G>(&self, grid: &G) -> Position<usize>
    where
        G: Grid<T>,
    {
        Position {
            x: self % grid.width(),
            y: self / grid.width(),
        }
    }
}
impl<T> GridIndex<T> for (usize, usize) {
    fn to_flat_index<G>(&self, grid: &G) -> usize
    where
        G: Grid<T>,
    {
        debug_assert!(self.0 < grid.height());
        debug_assert!(self.1 < grid.width());
        self.0 * grid.width() + self.1
    }

    fn to_coordinates<G>(&self, _grid: &G) -> (usize, usize)
    where
        G: Grid<T>,
    {
        *self
    }

    fn to_position<G>(&self, _: &G) -> Position<usize>
    where
        G: Grid<T>,
    {
        Position {
            x: self.1,
            y: self.0,
        }
    }
}
impl<T> GridIndex<T> for Position<usize> {
    fn to_flat_index<G>(&self, grid: &G) -> usize
    where
        G: Grid<T>,
    {
        debug_assert!(self.y < grid.height());
        debug_assert!(self.x < grid.width());
        self.y * grid.width() + self.x
    }

    fn to_coordinates<G>(&self, _grid: &G) -> (usize, usize)
    where
        G: Grid<T>,
    {
        self.as_yx_tuple()
    }

    fn to_position<G>(&self, _: &G) -> Position<usize>
    where
        G: Grid<T>,
    {
        *self
    }
}
