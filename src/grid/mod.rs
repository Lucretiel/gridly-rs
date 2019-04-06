pub mod views;

use core::marker::PhantomData;

use crate::location::component::{
    ColumnRange, ColumnRangeError, Range as IndexRange, RangeError, RowRange, RowRangeError,
};
use crate::location::{Column, Component as LocComponent, Location, Row};
use crate::vector::{Columns, Component as VecComponent, Rows, Vector};


use views::{GridView, GridSingleView};

/// High-level trait implementing grid sizes and boundary checking.
///
/// This trait doesn't provide any direct grid functionality, but instead
/// provides the bounds checking which is generic to all of the different
/// kinds of grid.
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
}

pub trait GridBoundsExt: GridBounds {
    /// Return the index of the leftmost row or column of this grid.
    #[inline]
    fn root_component<C: LocComponent>(&self) -> C {
        C::from_location(&self.root())
    }

    /// Get the height or width of this grid.
    #[inline]
    fn dimension<C: VecComponent>(&self) -> C {
        C::from_vector(&self.dimensions())
    }

    /// Get a Range over the row or column indexes
    #[inline]
    fn range<C: LocComponent>(&self) -> IndexRange<C> {
        IndexRange::span(self.root_component(), self.dimension())
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
    fn component_in_bounds<C: LocComponent>(&self, c: C) -> bool {
        self.range().in_bounds(c)
    }

    /// Check that a location is inside the bounds of this grid.
    ///
    /// Returns the Location if successful, or an error describing the boundary
    /// error if not. This function is intended to help write more expressive code;
    /// ie, `grid.check_location(loc).and_then(|loc| ...)`. Note that the
    /// safe grid interfaces are guarenteed to be bounds checked, where relevant.
    fn check_location(&self, loc: impl Into<Location>) -> Result<Location, LocationRangeError> {
        let loc = loc.into();
        self.check_component(loc.row)?;
        self.check_component(loc.column)?;
        Ok(loc)
    }

    /// Returns true if a locaton is inside the bounds of this grid.
    fn location_in_bounds(&self, location: impl Into<Location>) -> bool {
        self.check_location(location).is_ok()
    }
}

impl<G: GridBounds> GridBoundsExt for G {}

#[derive(Debug, Copy, Clone)]
pub enum LocationRangeError {
    Row(RowRangeError),
    Column(ColumnRangeError),
}

impl From<RowRangeError> for LocationRangeError {
    fn from(err: RowRangeError) -> Self {
        LocationRangeError::Row(err)
    }
}

impl From<ColumnRangeError> for LocationRangeError {
    fn from(err: ColumnRangeError) -> Self {
        LocationRangeError::Column(err)
    }
}

// TODO: Someday we'll have Generic Associated Types, at which point the Grid
// trait will become a lot more powerful (custom item wrapper types, specialized
// view types, etc). This might be possible today with some profoundly convoluted
// stuff, like SliceIndex::Output.
// TODO: for the love of god, find a way to deduplicate the immutable and mutable
// variants of everything. 2 traits, maybe? Perhaps unsafe casts under the hood?

pub trait Grid: GridBoundsExt {
    type Item;

    /// Get a reference to a cell, without doing bounds checking.
    unsafe fn get_unchecked(&self, loc: &Location) -> &Self::Item;
    unsafe fn get_unchecked_mut(&mut self, loc: &Location) -> &mut Self::Item;
    unsafe fn set_unchecked(&self, loc: &Location, value: Self::Item);

    /// Get a reference to a cell in a grid.
    fn get(&self, location: impl Into<Location>) -> Result<&Self::Item, LocationRangeError> {
        self.check_location(location)
            .map(move |loc| unsafe { self.get_unchecked(&loc) })
    }

    /// Get a mutable reference to a cell in a grid.
    fn get_mut(
        &mut self,
        location: impl Into<Location>,
    ) -> Result<&mut Self::Item, LocationRangeError> {
        self.check_location(location)
            .map(move |loc| unsafe { self.get_unchecked_mut(&loc) })
    }

    /// Set a value in a grid
    fn set(
        &self,
        location: impl Into<Location>,
        value: Self::Item,
    ) -> Result<(), LocationRangeError> {
        self.check_location(location)
            .map(move |loc| unsafe { self.set_unchecked(&loc, value) })
    }
}

pub trait GridExt: Grid {
    fn view<T: LocComponent>(&self) -> GridView<Self, T> {
        GridView {
            grid: self,
            index: PhantomData,
        }
    }
    fn rows(&self) -> RowsView<Self> {
        self.view()
    }
    fn columns(&self) -> ColumnsView<Self> {
        self.view()
    }

    unsafe fn single_view_unchecked<T: LocComponent>(&self, index: T) -> GridSingleView<Self, T> {
        GridSingleView {
            grid: self,
            cross: index,
        }
    }
    unsafe fn row_unchecked(&self, row: impl Into<Row>) -> RowView<Self> {
        self.single_view_unchecked(row.into())
    }
    unsafe fn column_unchecked(&self, column: impl Into<Column>) -> ColumnView<Self> {
        self.single_view_unchecked(column.into())
    }

    fn single_view<T: LocComponent>(
        &self,
        index: T,
    ) -> Result<GridSingleView<Self, T>, RangeError<T>> {
        self.check_component(index)
            .map(move |idx| unsafe { self.single_view_unchecked(idx) })
    }

    fn row(&self, row: impl Into<Row>) -> Result<RowView<Self>, RowRangeError> {
        self.single_view(row.into())
    }

    fn column(&self, column: impl Into<Column>) -> Result<ColumnView<Self>, ColumnRangeError> {
        self.single_view(column.into())
    }
}
