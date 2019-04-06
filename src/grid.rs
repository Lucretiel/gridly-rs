use crate::location::component::{
    ColumnRange, ColumnRangeError, Range as IndexRange, RangeError, RowRange, RowRangeError,
};
use crate::location::{Column, Component as LocComponent, Location, Range as LocRange, Row};
use crate::vector::{Columns, Component as VecComponent, Rows, Vector};
use core::iter::FusedIterator;
use core::marker::PhantomData;
use core::ops::Index;

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

// TODO: mutable views. Find a way to deuplicate all of this.
pub struct GridView<'a, G: Grid + ?Sized, T: LocComponent> {
    grid: &'a G,
    index: PhantomData<T>,
}

impl<'a, G: Grid + ?Sized, T: LocComponent> GridView<'a, G, T> {
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

impl<'a, G: Grid + ?Sized, T: LocComponent> IntoIterator for GridView<'a, G, T> {
    type IntoIter = GridIter<'a, G, T>;
    type Item = GridSingleView<'a, G, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'b, 'a, G: Grid + ?Sized, T: LocComponent> IntoIterator for &'b GridView<'a, G, T> {
    type IntoIter = GridIter<'a, G, T>;
    type Item = GridSingleView<'a, G, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An iterator over the rows or columns of a grid
pub struct GridIter<'a, G: Grid + ?Sized, T: LocComponent> {
    grid: &'a G,
    range: IndexRange<T>,
}

impl<'a, G: Grid + ?Sized, T: LocComponent> Iterator for GridIter<'a, G, T> {
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

impl<'a, G: Grid + ?Sized, T: LocComponent> DoubleEndedIterator for GridIter<'a, G, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range.next_back().map(|idx| GridSingleView {
            grid: self.grid,
            index: idx,
        })
    }
}

impl<'a, G: Grid + ?Sized, T: LocComponent> FusedIterator for GridIter<'a, G, T> {}
impl<'a, G: Grid + ?Sized, T: LocComponent> ExactSizeIterator for GridIter<'a, G, T> {}

// Implementor notes: a GridSingleView's index field is guaranteed to have been
// bounds checked against the grid. Therefore, we provide unsafe constructors, and
// then freely use unsafe accessors in the safe interface.
pub struct GridSingleView<'a, G: Grid + ?Sized, T: LocComponent> {
    grid: &'a G,
    index: T,
}

impl<'a, G: Grid + ?Sized, T: LocComponent> GridSingleView<'a, G, T> {
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

    pub fn iter(&self) -> GridSingleIterator<'a, G, T> {
        GridSingleIterator {
            grid: self.grid,
            range: self.grid.range().combine(self.index)
        }
    }
}

impl<'a, G: Grid + ?Sized, T: LocComponent> IntoIterator for GridSingleView<'a, G, T> {
    type IntoIter = GridSingleIterator<'a, G, T>;
    type Item = &'a G::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'b, 'a, G: Grid + ?Sized, T: LocComponent> IntoIterator for &'b GridSingleView<'a, G, T> {
    type IntoIter = GridSingleIterator<'a, G, T>;
    type Item = &'a G::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

type RowView<'a, G> = GridSingleView<'a, G, Row>;
type ColumnView<'a, G> = GridSingleView<'a, G, Column>;

/// An iterator over a single row or column of a grid
pub struct GridSingleIterator<'a, G: Grid + ?Sized, T: LocComponent> {
    grid: &'a G,
    range: LocRange<T::Converse>,
}

impl<'a, G: Grid + ?Sized, T: LocComponent> Iterator for GridSingleIterator<'a, G, T> {
    type Item = &'a G::Item;

    fn next(&mut self) -> Option<&'a G::Item> {
        self.range.next().map(|loc| unsafe { self.grid.get_unchecked(&loc)})
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }

    // TODO: more methods
}
