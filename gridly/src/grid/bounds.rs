use core::fmt::{self, Display, Formatter};

use crate::location::{Column, Component as LocComponent, Location, LocationLike, Row};
use crate::range::{
    ColumnRange, ColumnRangeError, ComponentRange, RangeError, RowRange, RowRangeError,
};
use crate::vector::{Columns, Component as VecComponent, Rows, Vector, VectorLike};

/// Grid trait implementing grid sizes and boundary checking.
///
/// This trait doesn't provide any direct grid storage functionality, but
/// instead provides the bounds checking which is generic to all of the
/// different kinds of grid.
///
/// All gridly grids have dimensions– the size of the grid in rows and
/// columns– and a root location, which is the location index of the top-left
/// cell of the grid (`(0, 0)` by default).
pub trait GridBounds {
    /// Get the dimensions of the grid, as a [`Vector`]. The dimensions of
    /// must be >= 0.
    #[must_use]
    fn dimensions(&self) -> Vector;

    /// Return the root location (ie, the top left) of the grid. All valid
    /// locations on the grid have `location >= root`. For most grids, this
    /// can just be [`Location`]`::`[`zero`]
    ///
    /// [`Location`]: Location
    /// [`zero`]: Location::zero
    #[must_use]
    fn root(&self) -> Location;

    /// Get the outer bound of the grid; that is, the location for which all
    /// valid locations in the grid have `row < outer.row && column < outer.column`
    #[inline]
    #[must_use]
    fn outer_bound(&self) -> Location {
        self.root() + self.dimensions()
    }

    /// Get the height of the grid in [`Rows`].
    #[inline]
    #[must_use]
    fn num_rows(&self) -> Rows {
        self.dimensions().rows
    }

    /// Get the width of the grid, in [`Columns`].
    #[inline]
    #[must_use]
    fn num_columns(&self) -> Columns {
        self.dimensions().columns
    }

    /// Get the height or width of this grid.
    #[inline]
    #[must_use]
    fn dimension<C: VecComponent>(&self) -> C {
        self.dimensions().get_component()
    }

    /// Return the index of the topmost row of this grid. For most grids,
    /// this is 0, but some grids may include negatively indexed locations,
    /// or even offsets.
    #[inline]
    #[must_use]
    fn root_row(&self) -> Row {
        self.root().row
    }

    /// Return the index of the leftmost column of this grid. For most grids,
    /// this is 0, but some grids may include negatively indexed locations,
    /// or even offsets.
    #[inline]
    #[must_use]
    fn root_column(&self) -> Column {
        self.root().column
    }

    /// Return the index of the leftmost column or topmost row of this grid.
    #[inline]
    #[must_use]
    fn root_component<C: LocComponent>(&self) -> C {
        self.root().get_component()
    }

    /// Get a range iterator over all the [`Row`] indexes in this grid
    #[inline]
    #[must_use]
    fn row_range(&self) -> RowRange {
        self.range()
    }

    /// Get a range iterator over all the [`Column`] indexes in this grid
    #[inline]
    #[must_use]
    fn column_range(&self) -> ColumnRange {
        self.range()
    }

    /// Get a range iterator over the row or column indexes
    #[inline]
    #[must_use]
    fn range<C: LocComponent>(&self) -> ComponentRange<C> {
        ComponentRange::span(self.root_component(), self.dimension())
    }

    /// Check that a [`Row`] or a [`Column`] is inside the bounds described
    /// by this grid. Returns the component if it's inside the bounds, or
    /// an error describing the violated boundary if not. This function is
    /// intended to help write more expressive code; ie,
    /// `grid.check_component(Row(10)).and_then(|row| ...)`.
    #[inline]
    fn check_component<C: LocComponent>(&self, c: C) -> Result<C, RangeError<C>> {
        self.range().check(c)
    }

    /// Check that a [`Row`] is inside the bounds described by this grid.
    /// Returns the component if it's inside the bounds, or an error
    /// describing the violated boundary if not. This function is intended
    /// to help write more expressive code; ie,
    /// `grid.check_row(10).and_then(|row| ...)`.
    #[inline]
    fn check_row(&self, row: impl Into<Row>) -> Result<Row, RowRangeError> {
        self.check_component(row.into())
    }

    /// Check that a [`Column`] is inside the bounds described by this grid.
    /// Returns the component if it's inside the bounds, or an error
    /// describing the violated boundary if not. This function is intended
    /// to help write more expressive code; ie,
    /// `grid.check_column(10).and_then(|row| ...)`.
    #[inline]
    fn check_column(&self, column: impl Into<Column>) -> Result<Column, ColumnRangeError> {
        self.check_component(column.into())
    }

    /// Returns true if a [`Row`] or [`Column`] is inside the bounds described
    /// by this grid.
    #[inline]
    #[must_use]
    fn component_in_bounds<C: LocComponent>(&self, c: C) -> bool {
        self.check_component(c).is_ok()
    }

    /// Returns true if a [`Row`] is inside the bounds described
    /// by this grid.
    #[inline]
    #[must_use]
    fn row_in_bounds(&self, row: impl Into<Row>) -> bool {
        self.component_in_bounds(row.into())
    }

    /// Returns true if a [`Column`] is inside the bounds described
    /// by this grid.
    #[inline]
    #[must_use]
    fn column_in_bounds(&self, column: impl Into<Column>) -> bool {
        self.component_in_bounds(column.into())
    }

    /// Check that a location is inside the bounds of this grid.
    ///
    /// Returns the [`Location`] if successful, or an error describing the boundary
    /// error if not. This function is intended to help write more expressive
    /// code; ie, `grid.check_location(loc).and_then(|loc| ...)`.
    #[inline]
    fn check_location(&self, location: impl LocationLike) -> Result<Location, BoundsError> {
        match (
            self.check_row(location.row()),
            self.check_column(location.column()),
        ) {
            (Err(row), Err(column)) => Err(BoundsError::Both { row, column }),
            (Err(row), Ok(_)) => Err(BoundsError::Row(row)),
            (Ok(_), Err(column)) => Err(BoundsError::Column(column)),
            (Ok(row), Ok(column)) => Ok(Location { row, column }),
        }
    }

    /// Returns true if a locaton is inside the bounds of this grid.
    #[inline]
    #[must_use]
    fn location_in_bounds(&self, location: impl LocationLike) -> bool {
        self.check_location(location).is_ok()
    }
}

impl<G: GridBounds> GridBounds for &G {
    #[inline]
    fn dimensions(&self) -> Vector {
        G::dimensions(self)
    }

    #[inline]
    fn root(&self) -> Location {
        G::root(self)
    }
}

impl<G: GridBounds> GridBounds for &mut G {
    #[inline]
    fn dimensions(&self) -> Vector {
        G::dimensions(self)
    }

    #[inline]
    fn root(&self) -> Location {
        G::root(self)
    }
}

/// An out-of-bounds error for a Location on a grid
///
/// This error is returned by methods that perform bounds checking to indicate
/// a failed bounds check. It includes the specific boundary or
/// boundaries that were violated.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BoundsError {
    /// The location's [`Row`] was out of bounds.
    Row(RowRangeError),

    /// The location's [`Column`] was out of bounds.
    Column(ColumnRangeError),

    /// Both the [`Row`] and the [`Column`] were out of bounds.
    Both {
        row: RowRangeError,
        column: ColumnRangeError,
    },
}

impl BoundsError {
    // TODO: fn part<T: Component>(&self) -> Option<&RangeError<T>>
    // This is probably not possible except by adding something like
    // FromBoundsError to Component and implementing it separately for Row
    // and Column. It's also probably not necessary.

    /// The row component of the boundary error, if applicable.
    ///
    /// ```
    /// use gridly::prelude::*;
    ///
    /// let row_error = RowRangeError::TooLow(Row(0));
    /// let col_error = ColumnRangeError::TooLow(Column(0));
    ///
    /// assert_eq!(BoundsError::Row(row_error).row(), Some(&row_error));
    /// assert_eq!(BoundsError::Column(col_error).row(), None);
    /// assert_eq!(
    ///     BoundsError::Both{row: row_error, column: col_error}.row(), Some(&row_error)
    /// );
    /// ```
    pub fn row(&self) -> Option<&RowRangeError> {
        use BoundsError::*;

        match self {
            Row(row) | Both { row, .. } => Some(row),
            _ => None,
        }
    }

    /// The column component of the boundary error, if applicable.
    ///
    /// ```
    /// use gridly::prelude::*;
    ///
    /// let row_error = RowRangeError::TooLow(Row(0));
    /// let col_error = ColumnRangeError::TooLow(Column(0));
    ///
    /// assert_eq!(BoundsError::Row(row_error).column(), None);
    /// assert_eq!(BoundsError::Column(col_error).column(), Some(&col_error));
    /// assert_eq!(
    ///     BoundsError::Both{row: row_error, column: col_error}.column(), Some(&col_error)
    /// );
    /// ```
    pub fn column(&self) -> Option<&ColumnRangeError> {
        use BoundsError::*;

        match self {
            Column(column) | Both { column, .. } => Some(column),
            _ => None,
        }
    }
}

impl From<RowRangeError> for BoundsError {
    #[inline]
    fn from(err: RowRangeError) -> Self {
        BoundsError::Row(err)
    }
}

impl From<ColumnRangeError> for BoundsError {
    #[inline]
    fn from(err: ColumnRangeError) -> Self {
        BoundsError::Column(err)
    }
}

impl Display for BoundsError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            BoundsError::Row(err) => write!(f, "Row out of bounds: {}", err),
            BoundsError::Column(err) => write!(f, "Column out of bounds: {}", err),
            BoundsError::Both { row, column } => write!(
                f,
                "Row and Column both out of bounds: {row}; {column}",
                row = row,
                column = column
            ),
        }
    }
}

// TODO: Add this when we figure out how to make it compatible with no_std
/*
impl<T: Component> Error for BoundsError {
    fn source(&self) -> Option<&(Error + 'static)> {
        match self {
            BoundsError::Row(err) => Some(err),
            BoundsError::Column(err) => Some(err),

            // TODO: ?????
            BoundsError::Both{row, column} => Some(row),
        }
    }
}
*/

#[cfg(test)]
mod tests {
    use crate::grid::bounds::*;

    /// Trivial struct that implements GridBounds for testing
    #[derive(Debug, Clone, Eq, PartialEq)]
    struct Window {
        root: Location,
        dimensions: Vector,
    }

    impl GridBounds for Window {
        #[inline]
        fn root(&self) -> Location {
            self.root
        }

        #[inline]
        fn dimensions(&self) -> Vector {
            self.dimensions
        }
    }

    const TEST_WINDOW: Window = Window {
        root: Location {
            row: Row(-5),
            column: Column(3),
        },
        dimensions: Vector {
            rows: Rows(10),
            columns: Columns(20),
        },
    };

    static TEST_ROWS: [(Row, Result<Row, RowRangeError>); 3] = [
        (Row(-10), Err(RangeError::TooLow(Row(-5)))),
        (Row(0), Ok(Row(0))),
        (Row(10), Err(RangeError::TooHigh(Row(5)))),
    ];

    static TEST_COLUMNS: [(Column, Result<Column, ColumnRangeError>); 3] = [
        (Column(0), Err(RangeError::TooLow(Column(3)))),
        (Column(10), Ok(Column(10))),
        (Column(50), Err(RangeError::TooHigh(Column(23)))),
    ];

    #[test]
    fn test_outer_bound() {
        assert_eq!(TEST_WINDOW.outer_bound(), Row(5) + Column(23));
    }

    #[test]
    fn test_num_rows() {
        assert_eq!(TEST_WINDOW.num_rows(), Rows(10));
    }

    #[test]
    fn test_num_columns() {
        assert_eq!(TEST_WINDOW.num_columns(), Columns(20));
    }

    #[test]
    fn test_dimensions() {
        let rows: Rows = TEST_WINDOW.dimension();
        let columns: Columns = TEST_WINDOW.dimension();

        assert_eq!(rows, Rows(10));
        assert_eq!(columns, Columns(20));
    }

    #[test]
    fn test_root_row() {
        assert_eq!(TEST_WINDOW.root_row(), Row(-5));
    }

    #[test]
    fn test_root_column() {
        assert_eq!(TEST_WINDOW.root_column(), Column(3));
    }

    #[test]
    fn test_root_component() {
        let rows: Row = TEST_WINDOW.root_component();
        let columns: Column = TEST_WINDOW.root_component();

        assert_eq!(rows, Row(-5));
        assert_eq!(columns, Column(3));
    }

    #[test]
    fn test_range() {
        assert_eq!(TEST_WINDOW.range(), RowRange::bounded(Row(-5), Row(5)));
        assert_eq!(
            TEST_WINDOW.range(),
            ColumnRange::bounded(Column(3), Column(23))
        );
    }

    #[test]
    fn test_check_component() {
        for &(row, expected) in &TEST_ROWS {
            assert_eq!(TEST_WINDOW.check_component(row), expected);
        }

        for &(column, expected) in &TEST_COLUMNS {
            assert_eq!(TEST_WINDOW.check_component(column), expected);
        }
    }

    #[test]
    fn test_check_row() {
        for &(row, expected) in &TEST_ROWS {
            assert_eq!(TEST_WINDOW.check_row(row), expected);
            assert_eq!(TEST_WINDOW.check_row(row.0), expected);
        }
    }

    #[test]
    fn test_check_column() {
        for &(column, expected) in &TEST_COLUMNS {
            assert_eq!(TEST_WINDOW.check_column(column), expected);
            assert_eq!(TEST_WINDOW.check_column(column.0), expected);
        }
    }

    #[test]
    fn test_component_in_bounds() {
        for &(row, expected) in &TEST_ROWS {
            assert_eq!(TEST_WINDOW.component_in_bounds(row), expected.is_ok());
        }

        for &(column, expected) in &TEST_COLUMNS {
            assert_eq!(TEST_WINDOW.component_in_bounds(column), expected.is_ok());
        }
    }

    #[test]
    fn test_row_in_bounds() {
        for &(row, expected) in &TEST_ROWS {
            assert_eq!(TEST_WINDOW.row_in_bounds(row), expected.is_ok());
            assert_eq!(TEST_WINDOW.row_in_bounds(row.0), expected.is_ok());
        }
    }

    #[test]
    fn test_column_in_bounds() {
        for &(column, expected) in &TEST_COLUMNS {
            assert_eq!(TEST_WINDOW.column_in_bounds(column), expected.is_ok());
            assert_eq!(TEST_WINDOW.column_in_bounds(column.0), expected.is_ok());
        }
    }

    #[test]
    fn test_check_location() {
        for &(row, expected_row_result) in &TEST_ROWS {
            for &(column, expected_column_result) in &TEST_COLUMNS {
                let location = row + column;
                let result = TEST_WINDOW.check_location(location);

                match result {
                    Ok(result_loc) => {
                        assert_eq!(result_loc, location);
                        assert!(expected_row_result.is_ok());
                        assert!(expected_column_result.is_ok());
                    }
                    Err(BoundsError::Row(row_err)) => {
                        assert_eq!(expected_row_result, Err(row_err));
                        assert!(expected_column_result.is_ok());
                    }
                    Err(BoundsError::Column(col_err)) => {
                        assert!(expected_row_result.is_ok());
                        assert_eq!(expected_column_result, Err(col_err));
                    }
                    Err(BoundsError::Both {
                        row: row_err,
                        column: col_err,
                    }) => {
                        assert_eq!(expected_row_result, Err(row_err));
                        assert_eq!(expected_column_result, Err(col_err));
                    }
                }
            }
        }
    }

    #[test]
    fn test_location_in_bounds() {
        for &(row, expected_row_result) in &TEST_ROWS {
            for &(column, expected_column_result) in &TEST_COLUMNS {
                let location = row + column;
                let expected = expected_row_result.is_ok() && expected_column_result.is_ok();

                assert_eq!(TEST_WINDOW.location_in_bounds(location), expected);
            }
        }
    }
}
