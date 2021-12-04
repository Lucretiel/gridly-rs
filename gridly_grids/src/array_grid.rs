use core::panic;
use std::{convert::TryInto, mem};

use gridly::prelude::*;

/**
A grid backed by stack-allocated arrays.

This grid is in row-major order; see [gridly-adapters](/gridly_adapters) for
a Transpose operator if you need column-major order.
*/
#[derive(Copy)]
pub struct ArrayGrid<T, const R: usize, const C: usize> {
    storage: [[T; C]; R],
}

impl<T, const R: usize, const C: usize> ArrayGrid<T, R, C> {
    #[inline]
    pub fn from_rows(rows: [[T; C]; R]) -> Self {
        Self { storage: rows }
    }

    pub fn new_fill_with(gen: impl Fn() -> T) -> Self {
        Self::from_rows(brownstone::build(|| brownstone::build(|| gen())))
    }

    pub fn new_with(gen: impl Fn(Location) -> T) -> Self {
        Self::from_rows(brownstone::build_indexed(|row| {
            brownstone::build_indexed(|column| gen(Location::new(row as isize, column as isize)))
        }))
    }

    /// Fill every cell in the grid with the values produced by repeatedly
    /// calling `gen`. Called in an unspecified order.
    pub fn fill_with(&mut self, gen: impl Fn() -> T) {
        self.storage
            .iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|cell| *cell = gen())
    }
}

impl<T: Default, const R: usize, const C: usize> ArrayGrid<T, R, C> {
    pub fn new() -> Self {
        Self::new_fill_with(T::default)
    }

    pub fn clear(&mut self) {
        self.fill_with(T::default)
    }
}

impl<T: Clone, const R: usize, const C: usize> ArrayGrid<T, R, C> {
    pub fn new_fill(value: &T) -> Self {
        Self::new_fill_with(|| value.clone())
    }

    pub fn fill(&mut self, value: &T) {
        self.fill_with(|| value.clone())
    }
}

impl<T, const R: usize, const C: usize> GridBounds for ArrayGrid<T, R, C> {
    fn dimensions(&self) -> Vector {
        // TODO: Find a way to static assert this.
        Vector {
            rows: Rows(R.try_into().expect("ArrayVec rows out of bounds")),
            columns: Columns(C.try_into().expect("ArrayVec columns out of bounds")),
        }
    }

    #[inline]
    fn root(&self) -> Location {
        Location::zero()
    }
}

impl<T, const R: usize, const C: usize> Grid for ArrayGrid<T, R, C> {
    type Item = T;

    #[inline]
    unsafe fn get_unchecked(&self, location: Location) -> &Self::Item {
        self.storage
            .get_unchecked(location.row.0 as usize)
            .get_unchecked(location.column.0 as usize)
    }
}

impl<T, const R: usize, const C: usize> GridMut for ArrayGrid<T, R, C> {
    #[inline]
    unsafe fn get_unchecked_mut(&mut self, location: Location) -> &mut Self::Item {
        self.storage
            .get_unchecked_mut(location.row.0 as usize)
            .get_unchecked_mut(location.column.0 as usize)
    }
}

impl<T, const R: usize, const C: usize> GridSetter for ArrayGrid<T, R, C> {
    #[inline]
    unsafe fn replace_unchecked(&mut self, location: Location, value: Self::Item) -> Self::Item {
        mem::replace(self.get_unchecked_mut(location), value)
    }

    #[inline]
    unsafe fn set_unchecked(&mut self, location: Location, value: Self::Item) {
        *self.get_unchecked_mut(location) = value
    }
}

impl<T: Default, const R: usize, const C: usize> Default for ArrayGrid<T, R, C> {
    fn default() -> Self {
        Self::from_rows(brownstone::build(|| brownstone::build(T::default)))
    }
}

impl<T: Clone, const R: usize, const C: usize> Clone for ArrayGrid<T, R, C> {
    fn clone(&self) -> Self {
        Self {
            storage: brownstone::build_indexed(|r| {
                brownstone::build_indexed(|c| self.storage[r][c].clone())
            }),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.storage
            .iter_mut()
            .zip(&source.storage)
            .flat_map(|(dest_row, src_row)| dest_row.iter_mut().zip(src_row))
            .for_each(|(dest, src)| dest.clone_from(src))
    }
}
