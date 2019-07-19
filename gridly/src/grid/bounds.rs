use core::fmt::{self, Display, Formatter};

use crate::location::{Column, Component as LocComponent, Location, Row, LocationLike};
use crate::range::{
    ColumnRange, ColumnRangeError, ComponentRange, RangeError, RowRange, RowRangeError,
};
use crate::vector::{Columns, Component as VecComponent, Rows, Vector, VectorLike};

/// High-level trait implementing grid sizes and boundary checking.
///
/// This trait doesn't provide any direct grid functionality, but instead
/// provides the bounds checking which is generic to all of the different
/// kinds of grid.
pub trait BaseGridBounds {
    /// Get the dimensions of the grid, as a [`Vector`]. This value MUST be
    /// const for any given grid.
    fn dimensions(&self) -> Vector;

    /// Return the root location (ie, the top left) of the grid. For most grids,
    /// this is (0, 0), but some grids may include negatively indexed locations,
    /// or even offsets. This value MUST be const for any given grid. All valid
    /// locations have `location >= root`.
    #[inline]
    fn root(&self) -> Location {
        Location::zero()
    }
}

impl<'a, G: BaseGridBounds + ?Sized> BaseGridBounds for &'a G {
    #[inline]
    fn dimensions(&self) -> Vector {
        (**self).dimensions()
    }

    #[inline]
    fn root(&self) -> Location {
        (**self).root()
    }
}

impl<'a, G: BaseGridBounds + ?Sized> BaseGridBounds for &'a mut G {
    #[inline]
    fn dimensions(&self) -> Vector {
        (**self).dimensions()
    }

    #[inline]
    fn root(&self) -> Location {
        (**self).root()
    }
}

pub trait GridBounds: BaseGridBounds {
    /// Get the outer root of the grid; that is, the location for which all
    /// valid locations in the grid have `row < outer.row && column < outer.column`
    #[inline]
    fn outer_root(&self) -> Location {
        self.root() + self.dimensions()
    }

    /// Get the height of the grid in [`Rows`]. This value MUST be const for
    /// any given grid.
    #[inline]
    fn num_rows(&self) -> Rows {
        self.dimensions().rows
    }

    /// Get the width of the grid, in [`Columns`]. This value MUST be const for
    /// any given grid.
    #[inline]
    fn num_columns(&self) -> Columns {
        self.dimensions().columns
    }

    /// Get the height or width of this grid.
    #[inline]
    fn dimension<C: VecComponent>(&self) -> C {
        self.dimensions().get_component()
    }

    /// Return the index of the topmost row of this grid. For most grids,
    /// this is 0, but some grids may include negatively indexed locations,
    /// or even offsets. This value MUST be const for any given grid.
    #[inline]
    fn root_row(&self) -> Row {
        self.root().row
    }

    /// Return the index of the leftmost column of this grid. For most grids,
    /// this is 0, but some grids may include negatively indexed locations,
    /// or even offsets. This value MUST be const for any given grid.
    #[inline]
    fn root_column(&self) -> Column {
        self.root().column
    }

    /// Return the index of the leftmost row or column of this grid.
    #[inline]
    fn root_component<C: LocComponent>(&self) -> C {
        self.root().get_component()
    }

    /// Get a Range over the row or column indexes
    #[inline]
    fn range<C: LocComponent>(&self) -> ComponentRange<C> {
        ComponentRange::span(self.root_component(), self.dimension())
    }

    /// A range iterator over all the column indexes in this grid
    #[inline]
    fn row_range(&self) -> RowRange {
        self.range()
    }

    /// A range iterator over all the row indexes in this grid
    #[inline]
    fn column_range(&self) -> ColumnRange {
        self.range()
    }

    /// Check that a Row or a Column is inside the bounds described by this Grid.
    #[inline]
    fn check_component<C: LocComponent>(&self, c: C) -> Result<C, RangeError<C>> {
        self.range().check(c)
    }

    #[inline]
    fn check_row(&self, row: impl Into<Row>) -> Result<Row, RowRangeError> {
        self.check_component(row.into())
    }

    #[inline]
    fn check_column(&self, column: impl Into<Column>) -> Result<Column, ColumnRangeError> {
        self.check_component(column.into())
    }

    #[inline]
    fn component_in_bounds<C: LocComponent>(&self, c: C) -> bool {
        self.check_component(c).is_ok()
    }

    #[inline]
    fn row_in_bounds(&self, row: impl Into<Row>) -> bool {
        self.component_in_bounds(row.into())
    }

    #[inline]
    fn column_in_bounds(&self, column: impl Into<Column>) -> bool {
        self.component_in_bounds(column.into())
    }

    /// Check that a location is inside the bounds of this grid.
    ///
    /// Returns the Location if successful, or an error describing the boundary
    /// error if not. This function is intended to help write more expressive code;
    /// ie, `grid.check_location(loc).and_then(|loc| ...)`. Note that the
    /// safe grid interfaces are guarenteed to be bounds checked, where relevant.
    fn check_location(&self, location: impl LocationLike) -> Result<Location, BoundsError> {
        let location = location.as_location();
        self.check_component(location.row)?;
        self.check_component(location.column)?;
        Ok(location)
    }

    /// Returns true if a locaton is inside the bounds of this grid.
    #[inline]
    fn location_in_bounds(&self, location: impl LocationLike) -> bool {
        self.check_location(location).is_ok()
    }
}

impl<G: BaseGridBounds> GridBounds for G {}

/// An out-of-bounds error for a Location on a grid
///
/// This error is returned by methods that perform bounds checking to indicate
/// a failed bounds check. It includes the specific boundary that was violated.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BoundsError {
    /// The location's `Row` was out of bounds
    Row(RowRangeError),

    /// The location's `Column` was out of bounds
    Column(ColumnRangeError),
}

impl From<RowRangeError> for BoundsError {
    fn from(err: RowRangeError) -> Self {
        BoundsError::Row(err)
    }
}

impl From<ColumnRangeError> for BoundsError {
    fn from(err: ColumnRangeError) -> Self {
        BoundsError::Column(err)
    }
}

impl Display for BoundsError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            BoundsError::Row(err) => write!(f, "Row out of bounds: {}", err),
            BoundsError::Column(err) => write!(f, "Column out of bounds: {}", err),
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
        }
    }
}
*/
