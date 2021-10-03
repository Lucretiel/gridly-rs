use crate::grid::GridBounds;
use crate::prelude::{Grid, GridMut, GridSetter, Location, Vector};

impl<T, const R: usize, const C: usize> GridBounds for [[T; C]; R] {
    #[inline]
    fn dimensions(&self) -> Vector {
        assert!(C <= core::isize::MAX as usize); // Column count overflow check
        assert!(R <= core::isize::MAX as usize); // Row count overflow check

        Vector::new(R as isize, C as isize)
    }

    #[inline]
    fn root(&self) -> Location {
        Location::new(0, 0)
    }
}

impl<T: Sized, const R: usize, const C: usize> Grid for [[T; C]; R] {
    type Item = T;

    #[inline]
    unsafe fn get_unchecked(&self, location: Location) -> &Self::Item {
        <[[T; C]]>::get_unchecked(self, location.row.0 as usize)
            .get_unchecked(location.column.0 as usize)
    }
}
impl<T, const R: usize, const C: usize> GridSetter for [[T; C]; R] {
    unsafe fn set_unchecked(&mut self, location: Location, value: Self::Item) {
        *<Self as GridMut>::get_unchecked_mut(self, location) = value;
    }

    unsafe fn replace_unchecked(&mut self, location: Location, value: Self::Item) -> Self::Item {
        core::mem::replace(<Self as GridMut>::get_unchecked_mut(self, location), value)
    }
}

impl<T, const R: usize, const C: usize> GridMut for [[T; C]; R] {
    #[inline]
    unsafe fn get_unchecked_mut(&mut self, location: Location) -> &mut Self::Item {
        <[[T; C]]>::get_unchecked_mut(self, location.row.0 as usize)
            .get_unchecked_mut(location.column.0 as usize)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_array() {
        // test array impls
        let mut arr = [[1i32, 2, 3], [4, 5, 6]];

        assert_eq!(arr.dimensions(), Vector::new(2, 3));
        assert_eq!(arr.root(), Location::new(0, 0));
        assert_eq!(arr.get(Location::new(1, 1)), Ok(&5));
        *arr.get_mut(Row(0) + Column(2)).expect("out of range") = 10;
        arr.set(Row(1) + Column(0), 20).expect("out of range");
        assert_eq!(arr, [[1, 2, 10], [20, 5, 6]]);
    }
}
