pub mod dense_grid;
pub mod grid_index;
pub mod sparse_grid;
use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

use grid_index::GridIndex;

use crate::helper::position::Direction8;

pub trait Grid<T>
where
    Self: Index<usize>
        + IndexMut<usize>
        + Index<(usize, usize)>
        + IndexMut<(usize, usize)>
        + IntoIterator
        + Clone
        + Debug,
{
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn get(&self, index: impl GridIndex<T>) -> Option<&T>;
    fn set(&mut self, index: impl GridIndex<T>, val: T) -> bool;
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a;
    /// returns the neigbors (if they exist) counterclockwise starting from the North
    fn get_neigbors4<'a>(
        &'a self,
        index: impl GridIndex<T>,
    ) -> impl Iterator<Item = (impl GridIndex<T>, &'a T)>
    where
        T: 'a,
        Self: Sized,
    {
        [
            get_north(self, index),
            get_east(self, index),
            get_south(self, index),
            get_west(self, index),
        ]
        .into_iter()
        .flatten()
    }
    /// returns the neigbors (if they exist) counterclockwise starting from the North
    fn get_neigbors8<'a>(
        &'a self,
        index: impl GridIndex<T>,
    ) -> impl Iterator<Item = (impl GridIndex<T>, &'a T)>
    where
        T: 'a,
        Self: Sized,
    {
        [
            get_north(self, index),
            get_north_east(self, index),
            get_east(self, index),
            get_south_east(self, index),
            get_south(self, index),
            get_south_west(self, index),
            get_west(self, index),
            get_north_west(self, index),
        ]
        .into_iter()
        .flatten()
    }
    fn get_dir8(&self, index: impl GridIndex<T>, dir: Direction8) -> Option<((usize, usize), &T)> {
        match dir {
            Direction8::North => get_north(self, index),
            Direction8::NorthEast => get_north_east(self, index),
            Direction8::East => get_east(self, index),
            Direction8::SouthEast => get_south_east(self, index),
            Direction8::South => get_south(self, index),
            Direction8::SouthWest => get_south_west(self, index),
            Direction8::West => get_west(self, index),
            Direction8::NorthWest => get_north_west(self, index),
        }
    }
}

#[inline(always)]
fn get_north<T>(grid: &impl Grid<T>, index: impl GridIndex<T>) -> Option<((usize, usize), &T)> {
    let (y, x) = index.to_coordinates(grid);
    if let Some(y) = y.checked_sub(1) {
        grid.get((y, x)).map(|r| ((y, x), r))
    } else {
        None
    }
}
#[inline(always)]
fn get_north_east<T>(
    grid: &impl Grid<T>,
    index: impl GridIndex<T>,
) -> Option<((usize, usize), &T)> {
    let (y, x) = index.to_coordinates(grid);
    y.checked_sub(1).and_then(|new_y| {
        let new_x = x + 1;
        if new_x < grid.width() {
            grid.get((new_y, new_x)).map(|r| ((new_y, new_x), r))
        } else {
            None
        }
    })
}
#[inline(always)]
fn get_east<T>(grid: &impl Grid<T>, index: impl GridIndex<T>) -> Option<((usize, usize), &T)> {
    let (y, x) = index.to_coordinates(grid);
    let x = x + 1;
    if x < grid.width() {
        grid.get((y, x)).map(|r| ((y, x), r))
    } else {
        None
    }
}
#[inline(always)]
fn get_south_east<T>(
    grid: &impl Grid<T>,
    index: impl GridIndex<T>,
) -> Option<((usize, usize), &T)> {
    let (y, x) = index.to_coordinates(grid);
    let x = x + 1;
    let y = y + 1;
    if x < grid.width() && y < grid.height() {
        grid.get((y, x)).map(|r| ((y, x), r))
    } else {
        None
    }
}
#[inline(always)]
fn get_south<T>(grid: &impl Grid<T>, index: impl GridIndex<T>) -> Option<((usize, usize), &T)> {
    let (y, x) = index.to_coordinates(grid);
    let y = y + 1;
    if y < grid.height() {
        grid.get((y, x)).map(|r| ((y, x), r))
    } else {
        None
    }
}
#[inline(always)]
fn get_south_west<T>(
    grid: &impl Grid<T>,
    index: impl GridIndex<T>,
) -> Option<((usize, usize), &T)> {
    let (y, x) = index.to_coordinates(grid);
    x.checked_sub(1).and_then(|new_x| {
        let new_y = y + 1;
        if new_y < grid.height() {
            grid.get((new_y, new_x)).map(|r| ((new_y, new_x), r))
        } else {
            None
        }
    })
}
#[inline(always)]
fn get_west<T>(grid: &impl Grid<T>, index: impl GridIndex<T>) -> Option<((usize, usize), &T)> {
    let (y, x) = index.to_coordinates(grid);
    if let Some(x) = x.checked_sub(1) {
        grid.get((y, x)).map(|r| ((y, x), r))
    } else {
        None
    }
}
#[inline(always)]
fn get_north_west<T>(
    grid: &impl Grid<T>,
    index: impl GridIndex<T>,
) -> Option<((usize, usize), &T)> {
    let (y, x) = index.to_coordinates(grid);
    x.checked_sub(1)
        .and_then(|new_x| y.checked_sub(1).map(|new_y| (new_y, new_x)))
        .and_then(|(new_y, new_x)| grid.get((new_y, new_x)).map(|r| ((new_y, new_x), r)))
}
