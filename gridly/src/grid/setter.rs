use crate::grid::bounds::BoundsError;
use crate::grid::BaseGrid;
use crate::location::Location;

pub trait BaseGridSetter: BaseGrid {
    // TODO: try_set_unchecked
    unsafe fn set_unchecked(&mut self, location: &Location, value: Self::Item);
}

impl<G: BaseGridSetter> BaseGridSetter for &mut G {
    unsafe fn set_unchecked(&mut self, location: &Location, value: Self::Item) {
        (**self).set_unchecked(location, value)
    }
}

pub trait GridSetter: BaseGridSetter {
    fn set(&mut self, location: impl Into<Location>, value: Self::Item) -> Result<(), BoundsError> {
        self.check_location(location)
            .map(move |loc| unsafe { self.set_unchecked(&loc, value) })
    }
}

impl<G: BaseGridSetter> GridSetter for G where Self::Item: Sized {}
