use crate::grid::bounds::BoundsError;
use crate::grid::view::Grid;
use crate::location::{Location, LocationLike};

pub trait BaseGridMut: Grid {
    // TODO: try_get_unchecked_mut
    unsafe fn get_unchecked_mut(&mut self, location: &Location) -> &mut Self::Item;
}

impl<G: BaseGridMut> BaseGridMut for &mut G {
    #[inline]
    unsafe fn get_unchecked_mut(&mut self, location: &Location) -> &mut Self::Item {
        G::get_unchecked_mut(self, location)
    }
}

pub trait GridMut: BaseGridMut {
    #[inline]
    fn get_mut(&mut self, location: impl LocationLike) -> Result<&mut Self::Item, BoundsError> {
        self.check_location(location)
            .map(move |loc| unsafe { self.get_unchecked_mut(&loc) })
    }
}

// TODO: mutable views, iterators
// TODO: modify this trait to support extra behavior when references are dropped
// (for instance, to allow clearing sparse grids). This will need to wait for
// HKTs or GATs

impl<G: BaseGridMut> GridMut for G {}
