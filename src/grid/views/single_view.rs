use core::iter::FusedIterator;
use core::ops::Index;

use crate::grid::Grid;
use crate::location::{Component, Row, Column, Range as LocRange};
use crate::location::component::{RangeError};

// Implementor notes: a GridSingleView's index field is guaranteed to have been
// bounds checked against the grid. Therefore, we provide unsafe constructors, and
// then freely use unsafe accessors in the safe interface.
pub struct GridSingleView<'a, G: Grid + ?Sized, T: Component> {
    grid: &'a G,
    index: T,
}

impl<'a, G: Grid + ?Sized, T: Component> GridSingleView<'a, G, T> {
    unsafe fn new_unchecked(grid: &'a G, index: T) -> Self {
        GridSingleView { grid, index }
    }

    fn new(grid: &'a G, index: impl Into<T>) -> Result<Self, RangeError<T>> {
        grid.check_component(index.into())
            .map(|index| unsafe { Self::new_unchecked(grid, index) })
    }

    pub fn index(&self) -> T {
        self.index
    }

    pub unsafe fn get_unchecked(&self, cross: T::Converse) -> &'a G::Item {
        self.grid.get_unchecked(&self.index.combine(cross))
    }

    pub fn get(
        &self,
        cross: impl Into<T::Converse>,
    ) -> Result<&'a G::Item, RangeError<T::Converse>> {
        self.grid
            .check_component(cross.into())
            .map(move |cross| unsafe { self.get_unchecked(cross) })
    }

    pub fn iter(&self) -> GridSingleIter<'a, G, T> {
        GridSingleIter {
            grid: self.grid,
            range: self.grid.range().combine(self.index)
        }
    }
}

impl<'a, G: Grid + ?Sized, T: Component> Index<T::Converse> for GridSingleView<'a, G, T> {
    type Output = G::Item;

    fn index(&self, idx: T::Converse) -> &G::Item {
        // TODO: insert error message once RangeError implements Error + Display
        self.get(idx).unwrap_or_else(|_err| panic!("{} out of range", T::name()))
    }
}

impl<'a, G: Grid + ?Sized, T: Component> IntoIterator for GridSingleView<'a, G, T> {
    type IntoIter = GridSingleIter<'a, G, T>;
    type Item = &'a G::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'b, 'a, G: Grid + ?Sized, T: Component> IntoIterator for &'b GridSingleView<'a, G, T> {
    type IntoIter = GridSingleIter<'a, G, T>;
    type Item = &'a G::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

type RowView<'a, G> = GridSingleView<'a, G, Row>;
type ColumnView<'a, G> = GridSingleView<'a, G, Column>;

/// An iterator over a single row or column of a grid
pub struct GridSingleIter<'a, G: Grid + ?Sized, T: Component> {
    grid: &'a G,
    range: LocRange<T::Converse>,
}

impl<'a, G: Grid + ?Sized, T: Component> Iterator for GridSingleIter<'a, G, T> {
    type Item = &'a G::Item;

    fn next(&mut self) -> Option<&'a G::Item> {
        self.range.next().map(|loc| unsafe { self.grid.get_unchecked(&loc)})
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }

    // TODO: more methods
}

impl<'a, G: Grid + ?Sized, T: Component> DoubleEndedIterator for GridSingleIter<'a, G, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range.next_back().map(|loc| unsafe {self.grid.get_unchecked(&loc)})
    }
}

impl<'a, G: Grid + ?Sized, T: Component> FusedIterator for GridSingleIter<'a, G, T> {}
impl<'a, G: Grid + ?Sized, T: Component> ExactSizeIterator for GridSingleIter<'a, G, T> {}
// TODO: TrustedLen
