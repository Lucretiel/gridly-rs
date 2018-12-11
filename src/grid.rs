use derive_more::*;

use crate::location::{Column, Component as LocComponent, Location, Row};
use crate::vector::{Columns, Rows, Vector};

/// Error indicating that a Row or Column was out of bounds.
///
///  Note that the bounds expressed in this error are half inclusive; that is,
///  the lower bound in TooLow is an inclusive lower bound, but the upper bound
///  in TooHigh is an exclusive upper bound. This is consistent with the
///  conventional range representation of `low..high`
#[derive(Debug, Copy, Clone)]
pub enum RangeError<T: LocComponent> {
    /// The given row or column was too low. The value in the error is the
    /// minimum row or column, inclusive.
    TooLow(T),

    /// The given row or column was too high. The given value in the error is
    /// the maximum row or column, exclusive (that is, a value *equal* to the
    /// error value is considered too high).
    TooHigh(T),
}

#[derive(Debug, Copy, Clone, From)]
pub enum LocationRangeError {
    Row(RangeError<Row>),
    Column(RangeError<Column>),
}

pub trait GridBounds {
    /// Return the index of the topmost row of this grid. For most grids,
    /// this is 0, but some grids may include negatively indexed locations,
    /// or even offsets. This value MUST be const for any given grid.
    fn root_row(&self) -> Row {
        Row(0)
    }

    /// Return the index of the leftmost column of this grid. For most grids,
    /// this is 0, but some grids may include negatively indexed locations,
    /// or even offsets. This value MUST be const for any given grid.
    fn root_column(&self) -> Column {
        Column(0)
    }

    /// Return the root location (ie, the top left) of the grid. For most grids,
    /// this is (0, 0), but some grids may include negatively indexed locations,
    /// or even offsets. This value MUST be const for any given grid.
    fn root(&self) -> Location {
        self.root_row() + self.root_column()
    }

    /// Get the height of the grid in [`Rows`]. This value MUST be const for
    /// any given grid.
    fn num_rows(&self) -> Rows;

    /// Get the width of the grid, in [`Columns`]. This value MUST be const for
    /// any given grid.
    fn num_columns(&self) -> Columns;

    /// Get the dimensions of the grid, as a [`Vector`]. This value MUST be
    /// const for any given grid.
    fn dimensions(&self) -> Vector {
        self.num_rows() + self.num_columns()
    }

    /// Check that a row is inside the bounds described by `root_row` and
    /// `num_rows`.
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

    /// Returns true if a row is inside the bounds described by `root_row` and
    /// `num_rows`
    fn row_in_bounds(&self, row: impl Into<Row>) -> bool {
        self.check_row(row).is_ok()
    }

    /// Check that a column is inside the bounds described by `root_columns` and
    /// `num_columns`.
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

    /// Returns true if a column is inside the bounds described by `root_column`
    /// and `num_columns`
    fn column_in_bounds(&self, column: impl Into<Column>) -> bool {
        self.check_column(column).is_ok()
    }

    /// Check that a location is inside the bounds of this grid. Returns the
    /// Location if successful, or an error describing the boundary error if not.
    fn check_location(&self, loc: impl Into<Location>) -> Result<Location, LocationRangeError> {
        let loc = loc.into();
        self.check_row(loc.row)?;
        self.check_column(loc.column)?;
        Ok(loc)
    }

    /// Returns true if a locaton is inside the bounds of this grid.
    fn location_in_bounds(&self, location: impl Into<Location>) -> bool {
        self.check_location(location).is_ok()
    }
}

pub trait Grid: GridBounds {
    type Item;

    /// Get a reference to a cell. This function assumes that all bounds
    /// checking has already been completed.
    unsafe fn get_unchecked(&self, loc: &Location) -> &Self::Item;

    fn get(&self, location: impl Into<Location>) -> Result<&Self::Item, LocationRangeError> {
        self.check_location(location)
            .map(move |loc| unsafe { self.get_unchecked(&loc) })
    }
}
