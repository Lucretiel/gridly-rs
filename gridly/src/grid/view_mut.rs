use crate::grid::bounds::BoundsError;
use crate::grid::view::Grid;
use crate::location::Location;

pub trait BaseGridMut: Grid {
    // TODO: try_get_unchecked_mut
    unsafe fn get_unchecked_mut(&mut self, location: &Location) -> &mut Self::Item;
}

impl<G: BaseGridMut> BaseGridMut for &mut G {
    unsafe fn get_unchecked_mut(&mut self, location: &Location) -> &mut Self::Item {
        (**self).get_unchecked_mut(location)
    }
}

pub trait GridMut: BaseGridMut {
    fn get_mut(&mut self, location: impl Into<Location>) -> Result<&mut Self::Item, BoundsError> {
        self.check_location(location)
            .map(move |loc| unsafe { self.get_unchecked_mut(&loc) })
    }
}

impl<G: BaseGridMut> GridMut for G {}
