use crate::grid::bounds::BoundsError;
use crate::grid::BaseGrid;
use crate::location::{Location, LocationLike};

pub trait BaseGridSetter: BaseGrid {
    unsafe fn replace_unchecked(&mut self, location: &Location, value: Self::Item) -> Self::Item;
    // TODO: try_set_unchecked
    unsafe fn set_unchecked(&mut self, location: &Location, value: Self::Item) {
        self.replace_unchecked(location, value);
    }
}

impl<G: BaseGridSetter> BaseGridSetter for &mut G {
    unsafe fn replace_unchecked(&mut self, location: &Location, value: Self::Item) -> Self::Item {
        (**self).replace_unchecked(location, value)
    }

    unsafe fn set_unchecked(&mut self, location: &Location, value: Self::Item) {
        (**self).set_unchecked(location, value)
    }
}

pub trait GridSetter: BaseGridSetter {
    fn replace(
        &mut self,
        location: impl LocationLike,
        value: Self::Item,
    ) -> Result<Self::Item, BoundsError> {
        self.check_location(location)
            .map(move |loc| unsafe { self.replace_unchecked(&loc, value) })
    }

    fn set(&mut self, location: impl LocationLike, value: Self::Item) -> Result<(), BoundsError> {
        self.check_location(location)
            .map(move |loc| unsafe { self.set_unchecked(&loc, value) })
    }
}

impl<G: BaseGridSetter> GridSetter for G where Self::Item: Sized {}
