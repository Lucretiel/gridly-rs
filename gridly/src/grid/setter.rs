use std::convert::Infallible;

use crate::grid::bounds::BoundsError;
use crate::grid::view::Grid;
use crate::location::Location;

pub trait BaseGridSetter: Grid {
    type SetError;

    unsafe fn try_set_unchecked(
        &mut self,
        location: &Location,
        value: Self::Item,
    ) -> Result<(), Self::SetError>;
}

impl<G: BaseGridSetter> BaseGridSetter for &mut G {
    type SetError = G::SetError;

    unsafe fn try_set_unchecked(
        &mut self,
        location: &Location,
        value: Self::Item,
    ) -> Result<(), Self::SetError> {
        (**self).try_set_unchecked(location, value)
    }
}

pub trait GridSetter: BaseGridSetter {
    fn try_set(
        &mut self,
        location: impl Into<Location>,
        value: Self::Item,
    ) -> Result<Result<(), Self::SetError>, BoundsError> {
        self.check_location(location.into())
            .map(move |loc| unsafe { self.try_set_unchecked(&loc, value) })
    }

    unsafe fn set_unchecked(&mut self, location: &Location, value: Self::Item)
    where
        Self: BaseGridSetter<SetError = Infallible>,
    {
        self.try_set_unchecked(location, value).unwrap()
    }

    fn set(&mut self, location: impl Into<Location>, value: Self::Item) -> Result<(), BoundsError>
    where
        Self: BaseGridSetter<SetError = Infallible>,
    {
        self.try_set(location, value).map(Result::unwrap)
    }
}

impl<G: BaseGridSetter> GridSetter for G {}
