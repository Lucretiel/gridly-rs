use crate::grid::{BaseGrid, BaseGridBounds, BaseGridSetter};
use crate::location::Location;
use crate::vector::Vector;

pub trait IntoTranslate: Sized {
    fn translate(self, translate: impl Into<Vector>) -> Translate<Self>;
}

impl<G: BaseGridBounds + Sized> IntoTranslate for G {
    fn translate(self, translate: impl Into<Vector>) -> Translate<Self> {
        Translate {
            grid: self,
            translation: translate.into(),
        }
    }
}

/// Translate a grid
#[derive(Debug, Clone, Default, Hash, Eq, PartialEq)]
pub struct Translate<G> {
    translation: Vector,
    grid: G,
}

impl<G> Translate<G> {
    /// Given a user location, get the translated location for the inner grid
    fn internal_index(&self, location: &Location) -> Location {
        *location - self.translation
    }
}

impl<G: BaseGridBounds> BaseGridBounds for Translate<G> {
    fn dimensions(&self) -> Vector {
        self.grid.dimensions()
    }

    fn root(&self) -> Location {
        self.grid.root() + self.translation
    }
}

impl<G: BaseGrid> BaseGrid for Translate<G> {
    type Item = G::Item;

    unsafe fn get_unchecked(&self, location: &Location) -> &Self::Item {
        self.grid.get_unchecked(&self.internal_index(location))
    }
}

impl<G: BaseGridSetter> BaseGridSetter for Translate<G> {
    unsafe fn set_unchecked(&mut self, location: &Location, value: Self::Item) {
        self.grid
            .set_unchecked(&self.internal_index(location), value)
    }
}
