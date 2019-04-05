use crate::location::{Column, Component as LocComponent, Location, Row, RowRange, ColumnRange, ComponentRange};
use crate::vector::{Columns, Rows, Vector, Component as VecComponent};

//TODO: separate type for dimensions; essentially an unsigned Vector
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

#[derive(Debug, Copy, Clone)]
pub enum LocationRangeError {
    Row(RangeError<Row>),
    Column(RangeError<Column>),
}

macro_rules! impl_range_err_from {
    ($Type:ident) => {
        impl From<RangeError<$Type>> for LocationRangeError {
            fn from(err: RangeError<$Type>) -> Self {
                LocationRangeError::$Type(err)
            }
        }
    };
}

impl_range_err_from!{Row}
impl_range_err_from!{Column}

/// High-level trait implementing grid sizes and boundary checking.
///
/// This trait doesn't provide any direct grid functionality, but instead
/// provides the bounds checking which is generic to all of the different
/// kinds of grid ([`Grid`], [`GridAdapter`]).
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

    fn root_component<C: LocComponent>(&self) -> C {
        C::from_location(&self.root())
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

    fn dimension<C: VecComponent>(&self) -> C {
        C::from_vector(&self.dimensions())
    }

    /// Get the dimensions of the grid, as a [`Vector`]. This value MUST be
    /// const for any given grid.
    fn dimensions(&self) -> Vector {
        self.num_rows() + self.num_columns()
    }

    fn row_range(&self) -> RowRange {
        RowRange::span(self.root_row(), self.num_rows())
    }

    fn column_range(&self) -> ColumnRange {
        ColumnRange::span(self.root_column(), self.num_columns())
    }

    fn range<C: LocComponent>(&self) -> ComponentRange<C> {
        ComponentRange::span(self.root_component(), self.num_component())
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

    /// Check that a location is inside the bounds of this grid.
    ///
    /// Returns the Location if successful, or an error describing the boundary
    /// error if not. This function is intended to help write more expressive code;
    /// ie, `grid.check_location(loc).and_then(|loc| ...)`. Note that the
    /// safe grid interfaces are guarenteed to be bounds checked, where relevant.
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

// TODO: Someday we'll have Generic Associated Types, at which point the Grid
// trait will become a lot more powerful (custom item wrapper types, specialized
// view types, etc). This might be possible today with some profoundly convoluted
// stuff, like SliceIndex::Output.
// TODO: for the love of god, find a way to deduplicate the immutable and mutable
// variants of everything. 2 traits, maybe? Perhaps unsafe casts under the hood?

pub trait Grid: GridBounds {
    type Item;

    /// Get a reference to a cell, without doing bounds checking.
    unsafe fn get_unchecked(&self, loc: &Location) -> &Self::Item;
    unsafe fn get_unchecked_mut(&mut self, loc: &Location) -> &mut Self::Item;

    /// Get a reference to a cell in a grid.
    fn get(&self, location: impl Into<Location>) -> Result<&Self::Item, LocationRangeError> {
        self.check_location(location)
            .map(move |loc| unsafe { self.get_unchecked(&loc) })
    }

    /// Get a mutable reference to a cell in a grid.
    fn get_mut(&mut self, location: impl Into<Location>) -> Result<&mut Self::Item, LocationRangeError> {
        self.check_location(location)
            .map(move |loc| unsafe { self.get_unchecked_mut(&loc) })
    }
}

