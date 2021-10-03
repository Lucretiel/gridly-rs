use crate::grid::bounds::BoundsError;
use crate::grid::view::Grid;
use crate::location::{Location, LocationLike};

pub trait GridMut: Grid {
    /// Get a mutable reference to a cell, without doing bounds checking.
    /// Implementors of this method are allowed to assume that bounds checking
    /// has already been performed on the location, which means that
    /// implementors are allowed to do their own unsafe `get_mut` operations
    /// on the underlying storage, where relevant / possible.
    ///
    /// # Safety
    ///
    /// Callers must ensure that the location has been bounds-checked before
    /// calling this method. The safe interface to `GridMut` automatically
    /// performs this checking for you.
    #[must_use]
    unsafe fn get_unchecked_mut(&mut self, location: Location) -> &mut Self::Item;

    /// Get a reference to a cell in a grid. Returns an error if the location
    /// is out of bounds with the specific boundary violation.
    #[inline]
    fn get_mut(&mut self, location: impl LocationLike) -> Result<&mut Self::Item, BoundsError> {
        self.check_location(location)
            .map(move |loc| unsafe { self.get_unchecked_mut(loc) })
    }
}

impl<G: GridMut> GridMut for &mut G {
    #[inline]
    unsafe fn get_unchecked_mut(&mut self, location: Location) -> &mut Self::Item {
        G::get_unchecked_mut(self, location)
    }

    #[inline]
    fn get_mut(&mut self, location: impl LocationLike) -> Result<&mut Self::Item, BoundsError> {
        G::get_mut(self, location)
    }
}

// TODO: mutable views, iterators. Feature parity with `Grid`
// TODO: modify this trait to support extra behavior when references are dropped
// (for instance, to allow clearing sparse grids). This will need to wait for
// HKTs or GATs
