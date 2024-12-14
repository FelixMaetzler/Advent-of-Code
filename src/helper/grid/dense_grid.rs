use std::ops::{Deref, DerefMut, Index, IndexMut};

use super::{grid_index::GridIndex, Grid};
#[derive(Clone)]
pub struct DenseGrid<T> {
    data: Vec<T>,
    height: usize,
    width: usize,
}
impl<T> Grid<T> for DenseGrid<T>
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
        self.data.get(index.to_flat_index(self))
    }

    fn set(&mut self, index: impl GridIndex<T>, val: T) -> bool {
        let i = index.to_flat_index(self);
        if i < self.data.len() {
            self.data[i] = val;
            true
        } else {
            false
        }
    }
}
impl<T> DenseGrid<T> {
    pub fn from_iter(it: impl Iterator<Item = T>, width: usize) -> Self {
        let data: Vec<_> = it.collect();
        let height = data.len() / width;
        debug_assert_eq!(height * width, data.len());
        Self {
            data,
            height,
            width,
        }
    }
    pub fn from_iter_iter(it: impl Iterator<Item = impl Iterator<Item = T>>) -> Self {
        let mut data = vec![];
        it.size_hint();
        let mut cols = None;
        for v in it {
            data.extend(v);
            if cols.is_none() {
                cols = Some(data.len());
            }
        }
        let width = cols.expect("grid is not empty");
        let height = data.len() / width;
        debug_assert_eq!(height * width, data.len());
        Self {
            data,
            height,
            width,
        }
    }
}
impl<T> Index<usize> for DenseGrid<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
impl<T> IndexMut<usize> for DenseGrid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
impl<T> Index<(usize, usize)> for DenseGrid<T>
where
    T: Clone,
{
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.to_flat_index(self)]
    }
}
impl<T> IndexMut<(usize, usize)> for DenseGrid<T>
where
    T: Clone,
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let index = index.to_flat_index(self);
        &mut self.data[index]
    }
}
impl<T> IntoIterator for DenseGrid<T> {
    type Item = T;

    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
impl<T> Deref for DenseGrid<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.data[..]
    }
}

impl<T> DerefMut for DenseGrid<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data[..]
    }
}
