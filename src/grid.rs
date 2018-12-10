use std::ops::{Index, IndexMut};

use derive_more::*;

use crate::location::{Location, Row, Column};
use crate::vector::{Vector, Rows, Columns};

#[derive(Debug, Copy, Clone)]
pub enum RangeError<T> {
    TooLow(T),
    TooHigh(T),
}

#[derive(Debug, Copy, Clone, From)]
pub enum LocationRangeError {
    Row(RangeError<Row>),
    Column(RangeError<Column>),
}

pub trait Grid {
    type Item;

    /// Get a reference to a cell. This function assumes that all bounds
    /// checking has already been completed.
    unsafe fn get_unchecked(&self, loc: &Location) -> &Self::Item;

    /// Get a mutable reference to a cell. This function assumes that all bounds
    /// checking has already been completed.
    unsafe fn get_unchecked_mut(&mut self, loc: &Location) -> &mut Self::Item;

    /// Collectively, root and dimensions are used for the safe function for
    /// bounds-checking, so make sure they're accurate.

    /// Return the index of the topmost row of this grid. For most grids,
    /// this is 0, but some grids may include negatively indexed locations,
    /// or even offsets. This value is const for any given grid.
    fn root_row(&self) -> Row;

    /// Return the index of the leftmost column of this grid. For most grids,
    /// this is 0, but some grids may include negatively indexed locations,
    /// or even offsets. This value is const for any given grid.
    fn root_column(&self) -> Column;

    /// Return the root location (ie, the top left) of the grid. For most grids,
    /// this is (0, 0), but some grids may include negatively indexed locations,
    /// or even offsets. This value is const for any given grid.
    fn root(&self) -> Location {
        self.root_row() + self.root_column()
    }

    /// Get the height of the grid in [`Rows`]. This value is const for any given
    /// grid.
    fn num_rows(&self) -> Rows;

    /// Get the width of the grid, in [`Columns`]. This value is const for any
    /// given grid.
    fn num_columns(&self) -> Columns;

    /// Get the dimensions of the grid, as a [`Vector`]. This value is const for
    /// any given grid.
    fn dimensions(&self) -> Vector {
        self.num_rows() + self.num_columns()
    }

    fn check_row(&self, row: impl Into<Row>) -> Result<Row, RangeError<Row>> {
        let row = row.into();
        let min_row = self.root_row();
        if row < min_row {
            return Err(RangeError::TooLow(min_row));
        }
        let max_row = min_row + self.num_rows();
        if row >= max_row {
            return Err(RangeError::TooHigh(max_row));
        }
        Ok(row)
    }

    fn row_in_bounds(&self, row: impl Into<Row>) -> bool {
        self.check_row(row).is_ok()
    }

    fn check_column(&self, column: impl Into<Column>) -> Result<Column, RangeError<Column>> {
        let column = column.into();
        let min_column = self.root_column();
        if column < min_column {
            return Err(RangeError::TooLow(min_column));
        }
        let max_column = min_column + self.num_columns();
        if column >= max_column {
            return Err(RangeError::TooHigh(max_column));
        }
        Ok(column)
    }

    fn column_in_bounds(&self, column: impl Into<Column>) -> bool {
        self.check_column(column).is_ok()
    }

    fn check_location(&self, loc: impl Into<Location>) -> Result<Location, LocationRangeError> {
        let loc = loc.into();
        self.check_row(loc.row)?;
        self.check_column(loc.column)?;
        Ok(loc)
    }

    fn location_in_bounds(&self, location: impl Into<Location>) -> bool {
        self.check_location(location).is_ok()
    }

    fn get(&self, location: impl Into<Location>) -> Result<&Self::Item, LocationRangeError> {
        self.check_location(location)
            .map(move |loc| unsafe { self.get_unchecked(&loc) })
    }

    fn get_mut(&mut self, location: impl Into<Location>) -> Result<&mut Self::Item, LocationRangeError> {
        self.check_location(location)
            .map(move |loc| unsafe { self.get_unchecked_mut(&loc) })
    }
}
