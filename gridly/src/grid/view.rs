use core::fmt::Debug;
use core::iter::FusedIterator;
use core::marker::PhantomData;
use core::ops::Index;

use crate::grid::{BoundsError, GridBounds};
use crate::location::{Column, Component as LocComponent, Location, LocationLike, Row};
use crate::range::{ColumnRangeError, ComponentRange, LocationRange, RangeError, RowRangeError};

/// Base Reader trait for grids.
///
/// This trait provides the grid's cell type, `Item`, and an unsafe getter
/// method for fetching a cell at a bounds-checked location. It uses this
/// unsafe getter, plus [`GridBounds`] based bounds-checking, to provide a
/// comprehensive and safe interface for reading and iterating over elements
/// in a grid.
pub trait Grid: GridBounds {
    /// The item type stored in the grid
    type Item;

    /// Get a reference to a cell, without doing bounds checking. Implementors
    /// of this method are allowed to assume that bounds checking has already
    /// been performed on the location, which means that implementors are allowed
    /// to do their own unsafe `get` operations on the underlying storage,
    /// where relevant / possible.
    unsafe fn get_unchecked(&self, location: &Location) -> &Self::Item;

    /// Get a reference to a cell in a grid. Returns an error if the location
    /// is out of bounds with the specific boundary violation.
    #[inline]
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

    /// Get a view of a single row in a grid, without bounds checking that row's index.
    #[inline]
    unsafe fn row_unchecked(&self, row: Row) -> RowView<Self> {
        self.single_view_unchecked(row)
    }

    /// Get a view of a single column in a grid, without bounds checking that column's index.
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

impl<G: Grid> Grid for &G {
    type Item = G::Item;

    #[inline]
    unsafe fn get_unchecked(&self, location: &Location) -> &Self::Item {
        G::get_unchecked(self, location)
    }
}

impl<G: Grid> Grid for &mut G {
    type Item = G::Item;

    #[inline]
    unsafe fn get_unchecked(&self, location: &Location) -> &Self::Item {
        G::get_unchecked(self, location)
    }
}

/// A view of the rows or columns in a grid.
///
/// This struct provides a row- or column-major view of a grid. For instance,
/// a `View<MyGrid, Row>` is a view of all of the rows in `MyGrid`. The view
/// can be indexed over its type (for instance, a `View<G, Row>` can be
/// indexed by [`Row`]). It can also be iterated, where each iteration step
/// produces a [`SingleView`], which is a view of a single row or column (that
/// single view can itself be iterated to get all the cells).
#[derive(Debug)]
pub struct View<'a, G: Grid + ?Sized, T: LocComponent> {
    grid: &'a G,
    index: PhantomData<T>,
}

impl<'a, G: Grid + ?Sized, T: LocComponent> View<'a, G, T> {
    /// Create a grid view. Grid views are pretty boring because they don't need
    /// to include anything besides the grid itself; the important stuff is
    /// encoded in the type.
    #[inline]
    fn new(grid: &'a G) -> Self {
        Self {
            grid,
            index: PhantomData,
        }
    }

    /// Get the length of this view; that is, the number of Rows or Columns
    #[inline]
    pub fn len(&self) -> T::Distance {
        self.grid.dimension()
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

// Custom clone implementation, because View is `Clone` even if G and T are not
impl<'a, G: Grid + ?Sized, T: LocComponent> Clone for View<'a, G, T> {
    fn clone(&self) -> Self {
        Self {
            grid: self.grid,
            index: PhantomData,
        }
    }
}

// TODO: impl Index for GridView. Requires Higher Kinded Lifetimes, because
// Index currently requires an &'a T, but we want to return a GridSingleView<'a, T>
// TODO: IntoIterator. We'd rather not maintain our own iterator type, so for
// now we require uses to use the iter() method.

/// A view over the rows of a grid.
///
/// See `View` for more details.
pub type RowsView<'a, G> = View<'a, G, Row>;

impl<'a, G: Grid + ?Sized> RowsView<'a, G> {
    #[inline]
    pub fn row(&self, row: impl Into<Row>) -> Result<RowView<'a, G>, RowRangeError> {
        self.get(row.into())
    }
}

/// A view over the columns of a grid.
///
/// See `View` for more details.
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
#[derive(Debug)]
pub struct SingleView<'a, G: Grid + ?Sized, T: LocComponent> {
    grid: &'a G,

    // Implementor notes: a GridSingleView's index field is guaranteed to
    // have been bounds checked against the grid. Therefore, we provide
    // unsafe and checked constructors, then freely use unsafe accessors
    // in the safe interface.
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

    /// Get the length of this view. For example, for a
    /// `SingleView<'a, G, Row>`, get the number of columns.
    #[inline]
    pub fn len(&self) -> <T::Converse as LocComponent>::Distance {
        self.grid.dimension()
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

        self.range()
            .map(move |loc| (loc.get_component(), unsafe { grid.get_unchecked(&loc) }))
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

// Manually implement clone, because T and G do not need to be clone.
impl<'a, G: Grid + ?Sized, T: LocComponent> Clone for SingleView<'a, G, T> {
    fn clone(&self) -> Self {
        Self {
            grid: self.grid,
            index: self.index,
        }
    }
}

/// A view of a single row of a grid.
///
/// See [`SingleView`] for more details.
pub type RowView<'a, G> = SingleView<'a, G, Row>;

impl<'a, G: Grid + ?Sized> RowView<'a, G> {
    /// Get a reference to the cell in a specific column of this view's row.
    #[inline]
    pub fn column(&self, column: impl Into<Column>) -> Result<&'a G::Item, ColumnRangeError> {
        self.get(column.into())
    }
}

/// A view of a single column of a grid.
///
/// See [`SingleView`] for more details.
pub type ColumnView<'a, G> = SingleView<'a, G, Column>;

impl<'a, G: Grid + ?Sized> ColumnView<'a, G> {
    /// Get a reference to the cell in a specific row of this view's column.
    #[inline]
    pub fn row(&self, row: impl Into<Row>) -> Result<&'a G::Item, RowRangeError> {
        self.get(row.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::BoundsError;
    use crate::prelude::*;
    use crate::range::{ColumnRangeError, RangeError, RowRangeError};

    // A stack-allocated grid with a fixed size of three rows by two columns.
    // The root of this grid is (-1, 0), which means that the valid rows are
    // [-1, 0, 1] and the valid columns are [0, 1]
    #[derive(Debug, Eq, PartialEq)]
    struct ThreeByTwo<T> {
        rows: [[T; 2]; 3],
    }

    impl<T> GridBounds for ThreeByTwo<T> {
        fn dimensions(&self) -> Vector {
            Vector::new(3, 2)
        }

        fn root(&self) -> Location {
            Location::new(-1, 0)
        }
    }

    impl<T> Grid for ThreeByTwo<T> {
        type Item = T;

        unsafe fn get_unchecked(&self, location: &Location) -> &T {
            // Normally we don't need to bounds check the location, but for
            // testing purposes we want to make sure that a location outside
            // the valid bounds never gets through.
            assert!(location.row.0 >= -1 && location.row.0 <= 1);
            assert!(location.column.0 >= 0 && location.column.0 <= 1);

            self.rows
                .get_unchecked((location.row.0 + 1) as usize)
                .get_unchecked(location.column.0 as usize)
        }
    }

    static TEST_GRID: ThreeByTwo<i16> = ThreeByTwo {
        rows: [[1, 2], [3, 4], [5, 6]],
    };

    static TEST_ROWS: [(Row, Option<RowRangeError>); 3] = [
        (Row(-10), Some(RangeError::TooLow(Row(-1)))),
        (Row(0), None),
        (Row(10), Some(RangeError::TooHigh(Row(2)))),
    ];

    static TEST_COLUMNS: [(Column, Option<ColumnRangeError>); 3] = [
        (Column(-10), Some(RangeError::TooLow(Column(0)))),
        (Column(0), None),
        (Column(10), Some(RangeError::TooHigh(Column(2)))),
    ];

    #[test]
    fn test_get_in_bounds() {
        let mut value = 1;

        for row in Row(-1).span(Rows(3)) {
            for column in Column(0).span(Columns(2)) {
                assert_eq!(TEST_GRID.get(row + column), Ok(&value));
                value += 1;
            }
        }
    }

    #[test]
    fn test_out_of_bounds() {
        for &(row, row_error) in &TEST_ROWS {
            for &(column, column_error) in &TEST_COLUMNS {
                let result = TEST_GRID.get(row + column);

                match result {
                    Err(BoundsError::Row(err)) => {
                        assert_eq!(row_error, Some(err));
                        assert_eq!(column_error, None);
                    }
                    Err(BoundsError::Column(err)) => {
                        assert_eq!(row_error, None);
                        assert_eq!(column_error, Some(err));
                    }
                    Err(BoundsError::Both { row, column }) => {
                        assert_eq!(row_error, Some(row));
                        assert_eq!(column_error, Some(column));
                    }
                    // We're only testing boundary errors here
                    Ok(_) => {}
                }
            }
        }
    }

    /*
    // Set of view and iterator tests that test the row, column, and generic
    // versions of all the relevant methods.
    macro_rules! view_test_suite {
        (
            $suite_name:ident :
            get_view: $get_view:ident,
            get_single_view: $get_single_view_from_grid:ident,
            get_single_view_from_view: $get_single_view_from_view:ident,
            get_cell_from_single_view: $get_cell_from_single_view:ident,
            get_len: $get_len:ident,
            get_root: $get_root:ident,
            Component: $Component:ident,
            Distance: $Distance:ident,
            Converse: $Converse:ident,
            Range: $Range:ident,
            RangeError: $RangeError:ident,
            ConverseRangeError: $ConverseRangeError:ident,
            View: $View:ident,
            SingleView: $SingleView:ident,
        ) => {
            mod $suite_name {
                use cool_asserts::assert_matches;

                #[allow(unused_imports)]
                use crate::prelude::*;
                #[allow(unused_imports)]
                use crate::range::{$Range, RowRangeError, ColumnRangeError, RangeError};
                use super::{TEST_GRID, ThreeByTwo};

                #[test]
                fn test_view() {
                    let min: $Component = TEST_GRID.$get_root();
                    let len: $Distance = TEST_GRID.$get_len();
                    let max: $Component = min + len;

                    let view = TEST_GRID.$get_view();

                    // For instance, assert view.len() == TEST_GRID.num_rows()
                    assert_eq!(view.len(), len);

                    // For instance, assert view.range() == RowRange(...)
                    assert_eq!(view.range(), $Range::span(min, len));

                    // For instance, assert row_view.get(Column(-10)) = Error(...)
                    assert_matches!(
                        view.$get_single_view_from_view($Component(-10)),
                        Err($RangeError::TooLow(m)) if m == min
                    );

                    assert_matches!(
                        view.$get_single_view_from_view($Component(10)),
                        Err($RangeError::TooHigh(m)) if m == max
                    );

                    // We have a set of more comprehensive SingleView tests later,
                    // so for now we just assert that they're constructed & bounds
                    // checked correctly
                    assert_matches!(
                        view.$get_single_view_from_view($Component(0)),
                        Ok(single_view) => {
                            assert_eq!(
                                single_view.grid as *const ThreeByTwo<i16>,
                                view.grid as *const ThreeByTwo<i16>
                            );
                            assert_eq!(single_view.index, $Component(0));
                        }
                    );
                }

                #[test]
                fn test_view_iter() {
                    let min: $Component = TEST_GRID.$get_root();
                    let len: $Distance = TEST_GRID.$get_len();
                    let max: $Component = min + len;

                    let view = TEST_GRID.$get_view();
                    let iter = view.iter();
                    let range = $Range::span(min, len);

                    for (single_view, index) in iter.zip(range) {
                        assert_eq!(
                            single_view.grid as *const ThreeByTwo<i16>,
                            view.grid as *const ThreeByTwo<i16>
                        );
                        assert_eq!(single_view.index, index);
                    }
                }

                #[test]
                fn test_single_view() {
                    let single_view = TEST_GRID.$get_single_view_from_grid($Component(0))
                        .expect("Unexpected bounds error");

                    assert_eq!(
                        single_view.grid as *const ThreeByTwo<i16>,
                        &TEST_GRID as *const ThreeByTwo<i16>
                    );
                    assert_eq!(single_view.index, $Component(0));

                    single_view.$get_cell_from_single_view($Converse(0));

                    assert_eq!(
                        single_view.$get_cell_from_single_view($Converse(-10)),
                        Err(RangeError::TooLow($Converse(-10)))
                    );

                    assert_eq!(
                        single_view.$get_cell_from_single_view($Converse(10)),
                        Err(RangeError::TooHigh($Converse(10)))
                    );

                    assert_eq!(single_view.$get_cell_from_single_view($Converse(0)), Ok(&3));
                }
            }
        }
    }

    view_test_suite! {
        test_row_views:
        get_view: rows,
        get_single_view: row,
        get_single_view_from_view: row,
        get_cell_from_single_view: column,
        get_len: num_rows,
        get_root: root_row,
        Component: Row,
        Distance: Rows,
        Converse: Column,
        Range: RowRange,
        RangeError: RowRangeError,
        ConverseRangeError: ColumnRangeError,
        View: RowsView,
        SingleView: RowView,
    }

    view_test_suite! {
        test_column_views:
        get_view: columns,
        get_single_view: column,
        get_single_view_from_view: column,
        get_cell_from_single_view: row,
        get_len: num_columns,
        get_root: root_column,
        Component: Column,
        Distance: Columns,
        Converse: Row,
        Range: ColumnRange,
        RangeError: ColumnRangeError,
        ConverseRangeError: RowRangeError,
        View: ColumnsView,
        SingleView: ColumnView,
    }
    */
}
