use core::fmt::Debug;
use core::iter::FusedIterator;
use core::marker::PhantomData;
use core::ops::Index;

use crate::grid::{BoundsError, GridBounds};
use crate::location::{Column, Component as LocComponent, Location, LocationLike, Row};
use crate::range::{ColumnRangeError, ComponentRange, LocationRange, RangeError, RowRangeError};

/// Base Reader trait for grids.
///
/// This trait provides the grid's cell type, `Item`, and a single, unsafe
/// reader function, `get_unchecked`, which provides a reference to a cell at a
/// location.
///
/// The [`Grid`] trait, which is automatically implemented for all [`BaseGrid`],
/// provides a safe and comprehensive interface to a `BaseGrid`, which includes
/// bounds checking based on [`GridBounds`] and many different view and iterator
/// methods.
pub trait BaseGrid: GridBounds {
    type Item;

    /// Get a reference to a cell, without doing bounds checking. Implementors
    /// of this method are allowed to assume that bounds checking has already
    /// been performed on the location, which means that implementors are allowed
    /// to do their own unsafe `get` operations on the underlying storage,
    /// where relevant.
    unsafe fn get_unchecked(&self, location: &Location) -> &Self::Item;
}

impl<G: BaseGrid> BaseGrid for &G {
    type Item = G::Item;

    #[inline]
    unsafe fn get_unchecked(&self, location: &Location) -> &Self::Item {
        G::get_unchecked(self, location)
    }
}

impl<G: BaseGrid> BaseGrid for &mut G {
    type Item = G::Item;

    #[inline]
    unsafe fn get_unchecked(&self, location: &Location) -> &Self::Item {
        G::get_unchecked(self, location)
    }
}

/// Trait for viewing the values in a grid
///
/// `Grid` provides a comprehensive interface for reading values in a grid. This
/// interface includes bounds-checked getters, iterators, and views.
pub trait Grid: BaseGrid {
    /// Get a reference to a cell in a grid. Returns an error if the location
    /// is out of bounds with the specific boundary violation.
    fn get(&self, location: impl LocationLike) -> Result<&Self::Item, BoundsError> {
        self.check_location(location)
            .map(move |loc| unsafe { self.get_unchecked(&loc) })
    }

    // Get a view of a grid, over its rows or columns. A view of a grid is
    // similar to a slice, but instead of being a view over specific elements,
    // it's a view over the rows and columns. See `[View]` for details.
    #[inline]
    fn view<T: LocComponent>(&self) -> View<Self, T> {
        View::new(self)
    }

    /// Get a view of a grid's rows. See `[View]` for details.
    #[inline]
    fn rows(&self) -> RowsView<Self> {
        self.view()
    }

    /// Get a view of a grid's columns. See `[View]` for details.
    #[inline]
    fn columns(&self) -> ColumnsView<Self> {
        self.view()
    }

    /// Get a view of a single row or column in a grid, without bounds checking that
    /// row or column index.
    #[inline]
    unsafe fn single_view_unchecked<T: LocComponent>(&self, index: T) -> SingleView<Self, T> {
        SingleView::new_unchecked(self, index)
    }

    /// Get a view of a single row in a grid, without bounds checking that row's index
    #[inline]
    unsafe fn row_unchecked(&self, row: Row) -> RowView<Self> {
        self.single_view_unchecked(row)
    }

    /// Get a view of a single column in a grid, without bounds checking that column's index
    #[inline]
    unsafe fn column_unchecked(&self, column: Column) -> ColumnView<Self> {
        self.single_view_unchecked(column)
    }

    /// Get a view of a single row or column in a grid. Returns an error if the index of the
    /// row or column is out of bounds for the grid.
    #[inline]
    fn single_view<T: LocComponent>(&self, index: T) -> Result<SingleView<Self, T>, RangeError<T>> {
        SingleView::new(self, index)
    }

    /// Get a view of a single row in a grid. Returns an error if the index of the row is
    /// out of bounds for the grid.
    #[inline]
    fn row(&self, row: impl Into<Row>) -> Result<RowView<Self>, RowRangeError> {
        self.single_view(row.into())
    }

    /// Get a view of a single column in a grid. Returns an error if the index of the column
    /// is out of bounds for the grid.
    #[inline]
    fn column(&self, column: impl Into<Column>) -> Result<ColumnView<Self>, ColumnRangeError> {
        self.single_view(column.into())
    }
}

impl<G: BaseGrid> Grid for G {}

/// A view of the Rows or Columns in a grid.
///
/// This struct provides a row- or column-major view of a grid. For instance,
/// a `View<MyGrid, Row>` is a View of all of the rows in MyGrid.
///
/// A view can be indexed over its major ordering. For example, a `View<G, Row>`
/// can be indexed over rows,
pub struct View<'a, G: Grid + ?Sized, T: LocComponent> {
    grid: &'a G,
    index: PhantomData<T>,
}

impl<'a, G: Grid + ?Sized, T: LocComponent> View<'a, G, T> {
    /// Create a grid view. Grid views are pretty boring because they don't need
    /// to include anything besides the grid itself.
    #[inline]
    fn new(grid: &'a G) -> Self {
        Self {
            grid,
            index: PhantomData,
        }
    }

    /// Get a view of a single row or column of the grid, without bounds checking
    /// the index.
    #[inline]
    pub unsafe fn get_unchecked(&self, index: T) -> SingleView<'a, G, T> {
        SingleView::new_unchecked(self.grid, index)
    }

    /// Get a view of a single row or column of the grid. Returns a range error
    /// if the index is out of range.
    #[inline]
    pub fn get(&self, index: T) -> Result<SingleView<'a, G, T>, RangeError<T>> {
        SingleView::new(self.grid, index)
    }

    /// Get a range over all the row or column indexes of this view.
    #[inline]
    pub fn range(&self) -> ComponentRange<T> {
        self.grid.range()
    }

    /// Create an iterator over the rows or columns of the grid. Each iterated
    /// element is a [`SingleView`], which is a view over a single row or column
    /// of the grid.
    #[inline]
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = SingleView<'a, G, T>>
           + DoubleEndedIterator
           + FusedIterator
           + ExactSizeIterator
           + Debug
           + Clone {
        let grid = self.grid;
        self.range()
            .map(move |index| unsafe { SingleView::new_unchecked(grid, index) })
    }
}

// TODO: impl Index for GridView. Requires Higher Kinded Lifetimes, because
// Index currently requires an &'a T, but we want to return a GridSingleView<'a, T>
// TODO: IntoIterator. We'd rather not maintain our own iterator type, so for
// now we require uses to use the iter() method.

/// A view over the rows of a grid.
pub type RowsView<'a, G> = View<'a, G, Row>;

impl<'a, G: Grid + ?Sized> RowsView<'a, G> {
    #[inline]
    pub fn row(&self, row: impl Into<Row>) -> Result<RowView<'a, G>, RowRangeError> {
        self.get(row.into())
    }
}

/// A view over the columns of a grid.
pub type ColumnsView<'a, G> = View<'a, G, Column>;

impl<'a, G: Grid + ?Sized> ColumnsView<'a, G> {
    #[inline]
    pub fn column(&self, column: impl Into<Column>) -> Result<ColumnView<'a, G>, ColumnRangeError> {
        self.get(column.into())
    }
}

/// View of a single Row or Column of a grid.
///
/// A `SingleView` provides a view over a single row or column of a grid, based
/// on its generic parameter. For instance, a SingleView<'a, G, Row> is a view
/// over a single row of a grid.
///
/// A `SingleView` can be indexed; for instance, a [`RowView`] can be indexed
/// with a [`Column`] to a get a specific cell.
pub struct SingleView<'a, G: Grid + ?Sized, T: LocComponent> {
    grid: &'a G,

    // Implementor notes: a GridSingleView's index field is guaranteed to have been
    // bounds checked against the grid. Therefore, we provide unsafe constructors, and
    // then freely use unsafe accessors in the safe interface.
    index: T,
}

impl<'a, G: Grid + ?Sized, T: LocComponent> SingleView<'a, G, T> {
    #[inline]
    unsafe fn new_unchecked(grid: &'a G, index: T) -> Self {
        Self { grid, index }
    }

    #[inline]
    fn new(grid: &'a G, index: T) -> Result<Self, RangeError<T>> {
        grid.check_component(index)
            .map(move |index| unsafe { Self::new_unchecked(grid, index) })
    }

    /// Get the index of the Row or Column that this view represents. This index
    /// is safely guaranteed to have been bounds checked when the `SingleView`
    /// was constructed.
    #[inline]
    pub fn index(&self) -> T {
        self.index
    }

    /// Get a particular cell in the row or column by an index, without bounds
    /// checking the index.
    #[inline]
    pub unsafe fn get_unchecked(&self, cross: T::Converse) -> &'a G::Item {
        self.grid.get_unchecked(&self.index.combine(cross))
    }

    /// Get a particular cell in the row or column, or return an error if the
    /// index is out of bounds.
    #[inline]
    pub fn get(&self, cross: T::Converse) -> Result<&'a G::Item, RangeError<T::Converse>> {
        self.grid
            .check_component(cross)
            .map(move |cross| unsafe { self.get_unchecked(cross) })
    }

    /// Get the specific locations associated with this view.
    #[inline]
    pub fn range(&self) -> LocationRange<T> {
        LocationRange::new(self.index, self.grid.range())
    }

    /// Get an iterator over the cells in this row or column
    #[inline]
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &'a G::Item>
           + DoubleEndedIterator
           + FusedIterator
           + ExactSizeIterator
           + Debug
           + Clone {
        let grid = self.grid;
        self.range()
            .map(move |loc| unsafe { grid.get_unchecked(&loc) })
    }

    /// Get an iterator over `(Location, &Item)` pairs for this row or column.
    #[inline]
    pub fn iter_with_locations(
        &self,
    ) -> impl Iterator<Item = (Location, &'a G::Item)>
           + DoubleEndedIterator
           + FusedIterator
           + ExactSizeIterator
           + Debug
           + Clone {
        let grid = self.grid;
        self.range()
            .map(move |loc| (loc, unsafe { grid.get_unchecked(&loc) }))
    }

    /// Get an iterator over `(Index, &Item)` pairs for this column. For instance,
    /// for a `RowView`, this iterates over `(Column, &Item)` pairs.
    #[inline]
    pub fn iter_with_indices(
        &self,
    ) -> impl Iterator<Item = (T::Converse, &'a G::Item)>
           + DoubleEndedIterator
           + FusedIterator
           + ExactSizeIterator
           + Debug
           + Clone {
        let grid = self.grid;
        let index = self.index;
        self.grid.range().map(move |cross: T::Converse| {
            (cross, unsafe { grid.get_unchecked(&cross.combine(index)) })
        })
    }
}

impl<'a, G: Grid + ?Sized, T: LocComponent> Index<T::Converse> for SingleView<'a, G, T> {
    type Output = G::Item;

    #[inline]
    fn index(&self, idx: T::Converse) -> &G::Item {
        self.get(idx).unwrap_or_else(|err| match err {
            RangeError::TooHigh(max) => panic!("{:?} too high, must be < {:?}", idx, max),
            RangeError::TooLow(min) => panic!("{:?} too low, must be >= {:?}", idx, min),
        })
    }
}

// TODO: IntoIterator

pub type RowView<'a, G> = SingleView<'a, G, Row>;

impl<'a, G: Grid + ?Sized> RowView<'a, G> {
    #[inline]
    pub fn column(&self, column: impl Into<Column>) -> Result<&'a G::Item, ColumnRangeError> {
        self.get(column.into())
    }
}

pub type ColumnView<'a, G> = SingleView<'a, G, Column>;

impl<'a, G: Grid + ?Sized> ColumnView<'a, G> {
    #[inline]
    pub fn row(&self, row: impl Into<Row>) -> Result<&'a G::Item, RowRangeError> {
        self.get(row.into())
    }
}
