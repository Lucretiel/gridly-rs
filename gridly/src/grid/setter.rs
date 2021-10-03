use crate::grid::bounds::BoundsError;
use crate::grid::Grid;
use crate::location::{Location, LocationLike};

/// Setter trait for grids. Allows setting and replacing elements in the grid.
/// Implementors should implement the unsafe [setter][GridSetter::set_unchecked]
/// and [replacer][GridSetter::replace_unchecked] methods. This trait then
/// provides implementations of safe, bounds-checked setters.
///
pub trait GridSetter: Grid {
    /// Replace the value at the given `location` with `value`, without
    /// bounds checking `location`. Returns the previous value in the grid.
    ///
    /// Implementors of this method are allowed to skip bounds checking
    /// `location`. The safe interface to [`GridSetter`] bounds checks its
    /// arguments where necessary before calling this method.
    ///
    /// # Safety
    ///
    /// Callers must ensure that the location has been bounds-checked before
    /// calling this method. The safe interface to `Grid` automatically performs
    /// this checking for you.
    #[must_use = "discarded return value of replace_unchecked; consider using set_unchecked"]
    unsafe fn replace_unchecked(&mut self, location: Location, value: Self::Item) -> Self::Item;

    /// Replace the value at the given `location` with `value`, without bounds
    /// checking `location`.
    ///
    /// Implementors of this method are allowed to skip bounds checking
    /// `location`. The safe interface to [`GridSetter`] bounds checks its
    /// arguments where necessary before calling this method.
    ///
    /// # Safety
    ///
    /// Callers must ensure that the location has been bounds-checked before
    /// calling this method. The safe interface to `Grid` automatically performs
    /// this checking for you.
    unsafe fn set_unchecked(&mut self, location: Location, value: Self::Item);

    /// Replace the value at the given `location` with `value`. Returns the
    /// previous value in the grid, or an error if the location was out of
    /// bounds.
    #[inline]
    fn replace(
        &mut self,
        location: impl LocationLike,
        value: Self::Item,
    ) -> Result<Self::Item, BoundsError> {
        self.check_location(location)
            .map(move |loc| unsafe { self.replace_unchecked(loc, value) })
    }

    /// Set the value at the given `location` in the grid. Returns an error
    /// if the location was out of bounds.
    #[inline]
    fn set(&mut self, location: impl LocationLike, value: Self::Item) -> Result<(), BoundsError> {
        self.check_location(location)
            .map(move |loc| unsafe { self.set_unchecked(loc, value) })
    }
}

impl<G: GridSetter> GridSetter for &mut G {
    #[inline]
    unsafe fn replace_unchecked(&mut self, location: Location, value: Self::Item) -> Self::Item {
        G::replace_unchecked(self, location, value)
    }

    #[inline]
    unsafe fn set_unchecked(&mut self, location: Location, value: Self::Item) {
        G::set_unchecked(self, location, value)
    }

    #[inline]
    fn replace(
        &mut self,
        location: impl LocationLike,
        value: Self::Item,
    ) -> Result<Self::Item, BoundsError> {
        G::replace(self, location, value)
    }

    #[inline]
    fn set(&mut self, location: impl LocationLike, value: Self::Item) -> Result<(), BoundsError> {
        G::set(self, location, value)
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::setter::*;
    use crate::prelude::*;
    use crate::range::RangeError;
    use core::mem::replace;

    /// A 2x2 grid in row-major order.
    #[derive(Debug, Clone, Default)]
    struct SimpleGrid<T> {
        cells: [T; 4],
    }

    impl<T> SimpleGrid<T> {
        fn index_of(loc: Location) -> usize {
            match (loc.row.0, loc.column.0) {
                (0, 0) => 0,
                (0, 1) => 1,
                (1, 0) => 2,
                (1, 1) => 3,
                _ => unreachable!(),
            }
        }
    }

    impl<T> GridBounds for SimpleGrid<T> {
        fn dimensions(&self) -> Vector {
            Vector::new(2, 2)
        }

        fn root(&self) -> Location {
            Location::zero()
        }
    }

    impl<T> Grid for SimpleGrid<T> {
        type Item = T;

        unsafe fn get_unchecked(&self, location: Location) -> &T {
            self.cells.get_unchecked(Self::index_of(location))
        }
    }

    impl<T> GridSetter for SimpleGrid<T> {
        unsafe fn replace_unchecked(&mut self, location: Location, value: T) -> T {
            replace(
                self.cells.get_unchecked_mut(Self::index_of(location)),
                value,
            )
        }

        unsafe fn set_unchecked(&mut self, location: Location, value: T) {
            *self.cells.get_unchecked_mut(Self::index_of(location)) = value;
        }
    }

    static TEST_ROWS: [(Row, Option<RowRangeError>); 3] = [
        (Row(-5), Some(RangeError::TooLow(Row(0)))),
        (Row(1), None),
        (Row(5), Some(RangeError::TooHigh(Row(2)))),
    ];

    static TEST_COLUMNS: [(Column, Option<ColumnRangeError>); 3] = [
        (Column(-10), Some(RangeError::TooLow(Column(0)))),
        (Column(0), None),
        (Column(10), Some(RangeError::TooHigh(Column(2)))),
    ];

    #[test]
    fn test_set() {
        let mut grid: SimpleGrid<Option<&'static str>> = SimpleGrid::default();

        for &(row, row_error) in &TEST_ROWS {
            for &(column, column_error) in &TEST_COLUMNS {
                let location = row + column;

                let result = grid.set(location, Some("Hello"));

                match (row_error, column_error) {
                    (Some(row), Some(column)) => {
                        assert_eq!(result, Err(BoundsError::Both { row, column }))
                    }
                    (Some(row), None) => assert_eq!(result, Err(BoundsError::Row(row))),
                    (None, Some(column)) => assert_eq!(result, Err(BoundsError::Column(column))),
                    (None, None) => assert_eq!(result, Ok(())),
                }
            }
        }

        assert_eq!(&grid.cells, &[None, None, Some("Hello"), None]);
    }

    #[test]
    fn test_replace() {
        let mut grid: SimpleGrid<Option<&'static str>> = SimpleGrid::default();
        grid.set((1, 0), Some("Hello")).unwrap();

        for &(row, row_error) in &TEST_ROWS {
            for &(column, column_error) in &TEST_COLUMNS {
                let location = row + column;

                let result = grid.replace(location, Some("World"));

                match (row_error, column_error) {
                    (Some(row), Some(column)) => {
                        assert_eq!(result, Err(BoundsError::Both { row, column }))
                    }
                    (Some(row), None) => assert_eq!(result, Err(BoundsError::Row(row))),
                    (None, Some(column)) => assert_eq!(result, Err(BoundsError::Column(column))),
                    (None, None) => assert_eq!(result, Ok(Some("Hello"))),
                }
            }
        }

        assert_eq!(&grid.cells, &[None, None, Some("World"), None]);
    }
}
