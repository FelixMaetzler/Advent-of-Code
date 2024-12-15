use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Index, IndexMut},
};

use super::{grid_index::GridIndex, Grid};
#[derive(Clone)]
pub struct SparseGrid<T> {
    data: HashMap<usize, T>,
    width: usize,
    height: usize,
}
impl<T> Grid<T> for SparseGrid<T>
where
    T: Clone,
{
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn get(&self, index: impl GridIndex<T>) -> Option<&T> {
        self.data.get(&index.to_flat_index(self))
    }
    fn set(&mut self, index: impl GridIndex<T>, val: T) -> bool {
        let i = index.to_flat_index(self);
        if i < self.height() * self.width() {
            self.data.insert(i, val);
            true
        } else {
            false
        }
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a,
    {
        self.data.values()
    }
}

impl<T> Index<usize> for SparseGrid<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.data.get(&index).expect("Index out of bounds")
    }
}

impl<T> IndexMut<usize> for SparseGrid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.data.get_mut(&index).expect("Index out of bounds")
    }
}
impl<T> Index<(usize, usize)> for SparseGrid<T>
where
    T: Clone,
{
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.data
            .get(&index.to_flat_index(self))
            .expect("Index out of bounds")
    }
}
impl<T> IndexMut<(usize, usize)> for SparseGrid<T>
where
    T: Clone,
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.data
            .get_mut(&index.to_flat_index(self))
            .expect("Index out of bounds")
    }
}
impl<T> IntoIterator for SparseGrid<T> {
    type Item = T;
    type IntoIter = std::collections::hash_map::IntoValues<usize, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_values()
    }
}
impl<T> Debug for SparseGrid<T> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
