use crate::grid::bounds::BoundsError;
use crate::grid::Grid;
use crate::location::{Location, LocationLike};

pub trait GridSetter: Grid {
    #[must_use = "discarded return value of replace_unchecked; consider using set_unchecked"]
    unsafe fn replace_unchecked(&mut self, location: &Location, value: Self::Item) -> Self::Item;
    // TODO: try_set_unchecked. Probably this should wait until HKT.
    #[inline]
    unsafe fn set_unchecked(&mut self, location: &Location, value: Self::Item) {
        // assign to a value to explicitly circumvent the must_use
        let _ = self.replace_unchecked(location, value);
    }

    #[inline]
    fn replace(
        &mut self,
        location: impl LocationLike,
        value: Self::Item,
    ) -> Result<Self::Item, BoundsError> {
        self.check_location(location)
            .map(move |loc| unsafe { self.replace_unchecked(&loc, value) })
    }

    #[inline]
    fn set(&mut self, location: impl LocationLike, value: Self::Item) -> Result<(), BoundsError> {
        self.check_location(location)
            .map(move |loc| unsafe { self.set_unchecked(&loc, value) })
    }
}

impl<G: GridSetter> GridSetter for &mut G {
    #[inline]
    unsafe fn replace_unchecked(&mut self, location: &Location, value: Self::Item) -> Self::Item {
        G::replace_unchecked(self, location, value)
    }

    #[inline]
    unsafe fn set_unchecked(&mut self, location: &Location, value: Self::Item) {
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

/*
#[cfg(test)]
mod tests {
    use core::mem::replace;
    use crate::prelude::*;
    use crate::grid::setter::*;
    /// A 2x2 grid
    #[derive(Debug, Clone, Default)]
    struct SimpleGrid<T> {
        cells: [T; 4],
    }

    impl<T> SimpleGrid<T> {
        unsafe fn index_of(loc: &Location) -> usize {
            match (loc.row.0, loc.column.0) {
                (0, 0) => 0,
                (0, 1) => 1,
                (1, 0) => 2,
                (1, 1) => 3,
                _ => unreachable!(),
            }
        }
    }

    impl<T> BaseGrid for SimpleGrid<T> {
        fn dimensions(&self) -> Vector {
            Vector::new(2, 2);
        }
    }

    impl<T> BaseGridSetter for SimpleGrid<T> {
        type Item = T;

        unsafe fn replace_unchecked(&mut self, location: &Location, value: T) -> T {
            replace(&mut self.cells[self.index_of(location)], value)
        }

        unsafe fn set_unchecked(&mut self, location: &Location, value: T) {
            self.cells[self.index_of(location)] = value;
        }
    }

    static TEST_ROWS: [(Row, Option<RowRangeError>); 3] = [
        (Row(-5), Some(RangeError::TooLow(Row(0)))),
        (Row(1), None),
        (Row(5), Some(RangeError::TooHigh(Row(2)))),
    ];

    static TEST_COLUMNS: [(Column, Option<ColumnRangeError>); 3] = [
        (Column(-10), Err(RangeError::TooLow(Column(0)))),
        (Column(0), None),
        (Column(10), Err(RangeError::TooHigh(Column(2)))),
    ];

    #[test]
    fn test_set() {
        let mut grid: SimpleGrid<Option<&'static str>> = SimpleGrid::default();

        for &(row, row_error) in &TEST_ROWS {
            for &(column, column_error) in &TEST_COLUMNS {
                let location = row + column;

                let result = grid.set(location, Some("HELLO"));

                match row_error {
                    Some(err) => assert_eq!(result, Err(BoundsError::Row(err))),
                    None => match column_error {
                        Some(err) => assert_eq!(result, Err(BoundsError::Column(err))),
                        None => assert_eq!(result, Ok(())),
                    }
                }
            }
        }

        assert_eq!(&grid.cells, &[None, None, Some("Hello", None)]);
    }
}
*/
