use crate::grid::bounds::BoundsError;
use crate::grid::view::Grid;
use crate::location::{Location, LocationLike};

pub trait GridMut: Grid {
    // TODO: try_get_unchecked_mut
    #[must_use]
    unsafe fn get_unchecked_mut(&mut self, location: &Location) -> &mut Self::Item;

    #[inline]
    fn get_mut(&mut self, location: impl LocationLike) -> Result<&mut Self::Item, BoundsError> {
        self.check_location(location)
            .map(move |loc| unsafe { self.get_unchecked_mut(&loc) })
    }
}

impl<G: GridMut> GridMut for &mut G {
    #[inline]
    unsafe fn get_unchecked_mut(&mut self, location: &Location) -> &mut Self::Item {
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
