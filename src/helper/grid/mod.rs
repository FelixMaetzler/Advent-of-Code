pub mod dense;
pub mod index;
pub mod sparse;
use core::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

use index::GridIndex;

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
    fn get<I>(&self, index: I) -> Option<&T>
    where
        I: GridIndex<T>;
    fn set<I>(&mut self, index: I, val: T) -> bool
    where
        I: GridIndex<T>;
    fn all_indices(&self) -> impl Iterator<Item = impl GridIndex<T>>;
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a;
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T>
    where
        T: 'a;
    /// returns the neigbors (if they exist) counterclockwise starting from the North.
    fn get_neigbors4<'a, I>(&'a self, index: I) -> impl Iterator<Item = (impl GridIndex<T>, &'a T)>
    where
        T: 'a,
        Self: Sized,
        I: GridIndex<T>,
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
    /// returns the neigbors (if they exist) counterclockwise starting from the North.
    fn get_neigbors8<'a, I>(&'a self, index: I) -> impl Iterator<Item = (impl GridIndex<T>, &'a T)>
    where
        T: 'a,
        Self: Sized,
        I: GridIndex<T>,
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
    fn get_dir8<I>(&self, index: I, dir: Direction8) -> Option<((usize, usize), &T)>
    where
        I: GridIndex<T>,
    {
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

fn get_north<T>(grid: &impl Grid<T>, index: impl GridIndex<T>) -> Option<((usize, usize), &T)> {
    let (y, x) = index.to_coordinates(grid);
    let y = y.checked_sub(1)?;
    grid.get((y, x)).map(|r| ((y, x), r))
}

fn get_north_east<T>(
    grid: &impl Grid<T>,
    index: impl GridIndex<T>,
) -> Option<((usize, usize), &T)> {
    let (y, x) = index.to_coordinates(grid);
    let new_y = y.checked_sub(1)?;
    let new_x = x + 1;
    if new_x < grid.width() {
        grid.get((new_y, new_x)).map(|r| ((new_y, new_x), r))
    } else {
        None
    }
}

fn get_east<T>(grid: &impl Grid<T>, index: impl GridIndex<T>) -> Option<((usize, usize), &T)> {
    let (y, x) = index.to_coordinates(grid);
    let x = x + 1;
    if x < grid.width() {
        grid.get((y, x)).map(|r| ((y, x), r))
    } else {
        None
    }
}

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

fn get_south<T>(grid: &impl Grid<T>, index: impl GridIndex<T>) -> Option<((usize, usize), &T)> {
    let (y, x) = index.to_coordinates(grid);
    let y = y + 1;
    if y < grid.height() {
        grid.get((y, x)).map(|r| ((y, x), r))
    } else {
        None
    }
}

fn get_south_west<T>(
    grid: &impl Grid<T>,
    index: impl GridIndex<T>,
) -> Option<((usize, usize), &T)> {
    let (y, x) = index.to_coordinates(grid);
    let new_x = x.checked_sub(1)?;
    let new_y = y + 1;
    if new_y < grid.height() {
        grid.get((new_y, new_x)).map(|r| ((new_y, new_x), r))
    } else {
        None
    }
}

fn get_west<T>(grid: &impl Grid<T>, index: impl GridIndex<T>) -> Option<((usize, usize), &T)> {
    let (y, x) = index.to_coordinates(grid);
    let x = x.checked_sub(1)?;
    grid.get((y, x)).map(|r| ((y, x), r))
}

fn get_north_west<T>(
    grid: &impl Grid<T>,
    index: impl GridIndex<T>,
) -> Option<((usize, usize), &T)> {
    let (y, x) = index.to_coordinates(grid);
    let (new_y, new_x) = x
        .checked_sub(1)
        .and_then(|new_x| y.checked_sub(1).map(|new_y| (new_y, new_x)))?;
    grid.get((new_y, new_x)).map(|r| ((new_y, new_x), r))
}
