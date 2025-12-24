use crate::helper::position::Position;

use super::{Grid, index::GridIndex};
use core::fmt::Write as _;
use core::{
    fmt::Debug,
    ops::{Index, IndexMut},
};
#[derive(Clone, PartialEq, Eq, Hash)]
#[expect(clippy::module_name_repetitions, reason = "makes more sense")]
pub struct DenseGrid<T> {
    data: Vec<T>,
    height: usize,
    width: usize,
}
impl<T> Grid<T> for DenseGrid<T>
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
    fn get<I>(&self, index: I) -> Option<&T>
    where
        I: GridIndex<T>,
    {
        self.data.get(index.to_flat_index(self))
    }

    fn set<I>(&mut self, index: I, val: T) -> bool
    where
        I: GridIndex<T>,
    {
        let i = index.to_flat_index(self);
        if i < self.data.len() {
            self.data[i] = val;
            true
        } else {
            false
        }
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a,
    {
        self.data.iter()
    }
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T>
    where
        T: 'a,
    {
        self.data.iter_mut()
    }
    fn all_indices(&self) -> impl Iterator<Item = impl GridIndex<T>> {
        0..self.len()
    }
}
impl<T> DenseGrid<T> {
    pub fn from_iter<I>(it: I, width: usize) -> Self
    where
        I: Iterator<Item = T>,
    {
        let data: Vec<_> = it.collect();
        let height = data.len() / width;
        debug_assert_eq!(height * width, data.len());
        Self {
            data,
            height,
            width,
        }
    }
    pub fn from_iter_iter<O, I>(it: O) -> Self
    where
        O: Iterator<Item = I>,
        I: Iterator<Item = T>,
    {
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
    pub fn from_string(s: &str) -> Self
    where
        T: TryFrom<char>,
        T::Error: Debug,
    {
        Self::from_iter_iter(
            s.lines()
                .map(|l| l.chars().map(|c| T::try_from(c).unwrap())),
        )
    }
    pub fn new(width: usize, height: usize, default: T) -> Self
    where
        T: Clone,
    {
        Self {
            data: vec![default; height * width],
            height,
            width,
        }
    }
    pub fn get_row(&self, n: usize) -> impl DoubleEndedIterator<Item = &T> {
        let start = n * self.width;
        let end = start + self.width;
        self.data[start..end].iter()
    }

    pub fn get_col(&self, x: usize) -> impl Iterator<Item = &T> {
        self.data[x..].iter().step_by(self.width)
    }

    pub fn set_row(&mut self, i: usize, row: &[T])
    where
        T: Clone,
    {
        assert_eq!(row.len(), self.width, "row length must match grid width");
        let start = i * self.width;
        let end = start + self.width;
        self.data[start..end].clone_from_slice(row);
    }
    pub fn set_col(&mut self, x: usize, col: &[T])
    where
        T: Clone,
    {
        assert_eq!(
            col.len(),
            self.height,
            "column length must match grid height"
        );
        for (y, item) in col.iter().enumerate() {
            self.data[y * self.width + x] = item.clone();
        }
    }
    pub fn split_width(&self, sub_width: usize) -> Vec<Self>
    where
        T: Clone,
    {
        debug_assert!(
            self.width.is_multiple_of(sub_width),
            "sub_width must divide evenly into width"
        );
        let cols = self.width / sub_width;

        let mut result = Vec::with_capacity(cols);

        for c in 0..cols {
            let mut data = Vec::with_capacity(sub_width * self.height);

            for y in 0..self.height {
                let start = y * self.width + c * sub_width;
                let end = start + sub_width;
                data.extend_from_slice(&self.data[start..end]);
            }

            result.push(Self {
                width: sub_width,
                height: self.height,
                data,
            });
        }

        result
    }
    pub fn remove_col(&mut self, col: usize) {
        assert!(col < self.width, "column out of bounds");

        for row in (0..self.height).rev() {
            let idx = row * self.width + col;
            self.data.remove(idx);
        }

        self.width -= 1;
    }
    pub fn transpose(&mut self)
    where
        T: Clone,
    {
        let mut out = Self {
            height: self.width,
            width: self.height,
            data: vec![self.data[0].clone(); self.data.len()],
        };

        for r in 0..self.height {
            for c in 0..self.width {
                out.data[c * out.width + r] = self.data[r * self.width + c].clone();
            }
        }

        *self = out;
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
    T: Clone + Debug,
{
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.to_flat_index(self)]
    }
}
impl<T> IndexMut<(usize, usize)> for DenseGrid<T>
where
    T: Clone + Debug,
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let index = index.to_flat_index(self);
        &mut self.data[index]
    }
}
impl<T> Index<Position<usize>> for DenseGrid<T>
where
    T: Clone + Debug,
{
    type Output = T;

    fn index(&self, index: Position<usize>) -> &Self::Output {
        &self.data[index.to_flat_index(self)]
    }
}
impl<T> IndexMut<Position<usize>> for DenseGrid<T>
where
    T: Clone + Debug,
{
    fn index_mut(&mut self, index: Position<usize>) -> &mut Self::Output {
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

impl<T> Debug for DenseGrid<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = self
            .data
            .chunks(self.width)
            .map(|c| {
                c.iter().fold(String::new(), |mut output, x| {
                    let _ = write!(output, "{x:?}");
                    output
                })
            })
            .fold(String::new(), |mut output, s| {
                let _ = writeln!(output, "{s}");
                output
            });
        f.write_fmt(format_args!("\n{s}"))
    }
}
