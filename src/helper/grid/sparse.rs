use std::{
    collections::{HashMap, hash_map::Iter},
    fmt::Debug,
    ops::{Index, IndexMut},
};

use super::{Grid, index::GridIndex};
#[derive(Clone, PartialEq, Eq)]
pub struct SparseGrid<T> {
    data: HashMap<usize, T>,
    width: usize,
    height: usize,
}
impl<T> Grid<T> for SparseGrid<T>
where
    T: Clone + Debug,
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

    fn all_indices(&self) -> impl Iterator<Item = impl GridIndex<T>> {
        self.data.keys().copied()
    }
}
impl<T> SparseGrid<T>
where
    T: Debug,
{
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: HashMap::new(),
            width,
            height,
        }
    }
    pub fn from_it<I>(it: I) -> Self
    where
        T: Clone,
        I: Iterator<Item = ((usize, usize), T)> + Clone,
    {
        let width = it.clone().map(|((_, x), _)| x).max().unwrap() + 1;
        let height = it.clone().map(|((y, _), _)| y).max().unwrap() + 1;
        let mut g = Self::new(width, height);
        it.for_each(|(k, v)| {
            g.set(k, v);
        });
        g
    }
    pub fn iter_all(&self) -> Iter<'_, usize, T> {
        self.data.iter()
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
    T: Clone + Debug,
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
    T: Clone + Debug,
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
impl<T> Debug for SparseGrid<T>
where
    T: Clone + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get((y, x)) {
                    Some(x) => s.extend(format!("{:?}", x.clone()).chars()),
                    None => s.push(' '),
                }
            }
            s.push('\n');
        }
        f.write_str(&s)
    }
}
