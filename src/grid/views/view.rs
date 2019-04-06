use core::marker::PhantomData;
use core::iter::FusedIterator;

use crate::grid::Grid;
use crate::grid::views::GridSingleView;
use crate::location::Component;
use crate::location::component::{RangeError, Range as IndexRange};

// TODO: mutable views. Find a way to deuplicate all of this.
pub struct GridView<'a, G: Grid + ?Sized, T: Component> {
    grid: &'a G,
    index: PhantomData<T>,
}

impl<'a, G: Grid + ?Sized, T: Component> GridView<'a, G, T> {
    unsafe fn get_unchecked(&self, cross: T) -> GridSingleView<G, T> {
        GridSingleView {
            grid: self.grid,
            cross,
        }
    }

    fn get(&self, cross: impl Into<T>) -> Result<GridSingleView<G, T>, RangeError<T>> {
        self.grid
            .range()
            .check(cross.into())
            .map(move |cross| unsafe { self.get_unchecked(cross) })
    }

    fn iter(&self) -> GridIter<'a, G, T> {
        GridIter {
            grid: self.grid,
            range: self.grid.range(),
        }
    }
}

// TODO: impl Index for GridView. Requires Higher Kinded Lifetimes, because
// Index currently requires an &'a T, but we want to return a GridSingleView<'a, T>

impl<'a, G: Grid + ?Sized, T: Component> IntoIterator for GridView<'a, G, T> {
    type IntoIter = GridIter<'a, G, T>;
    type Item = GridSingleView<'a, G, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'b, 'a, G: Grid + ?Sized, T: Component> IntoIterator for &'b GridView<'a, G, T> {
    type IntoIter = GridIter<'a, G, T>;
    type Item = GridSingleView<'a, G, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An iterator over the rows or columns of a grid
pub struct GridIter<'a, G: Grid + ?Sized, T: Component> {
    grid: &'a G,
    range: IndexRange<T>,
}

impl<'a, G: Grid + ?Sized, T: Component> Iterator for GridIter<'a, G, T> {
    type Item = GridSingleView<'a, G, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().map(|index| GridSingleView {
            grid: self.grid,
            index,
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }

    // TODO: other iterator methods
}

impl<'a, G: Grid + ?Sized, T: Component> DoubleEndedIterator for GridIter<'a, G, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range.next_back().map(|idx| GridSingleView {
            grid: self.grid,
            index: idx,
        })
    }
}

impl<'a, G: Grid + ?Sized, T: Component> FusedIterator for GridIter<'a, G, T> {}
impl<'a, G: Grid + ?Sized, T: Component> ExactSizeIterator for GridIter<'a, G, T> {}
// TODO: TrustedLen
