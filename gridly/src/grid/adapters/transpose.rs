use crate::grid::{BaseGrid, BaseGridBounds, BaseGridMut, BaseGridSetter};
use crate::location::Location;
use crate::vector::Vector;

pub trait IntoTranspose: Sized {
    fn transpose(self) -> Transpose<Self>;
}

impl<G: BaseGridBounds + Sized> IntoTranspose for G {
    fn transpose(self) -> Transpose<Self> {
        Transpose { grid: self }
    }
}

/// Transpose a grid, swapping its rows and columns
#[derive(Debug, Clone, Default, Hash, Eq, PartialEq)]
pub struct Transpose<G> {
    grid: G,
}

impl<G> Transpose<G> {
    pub fn transpose(self) -> G {
        self.grid
    }
}

impl<G: BaseGridBounds> BaseGridBounds for Transpose<G> {
    fn dimensions(&self) -> Vector {
        self.grid.dimensions().transpose()
    }

    fn root(&self) -> Location {
        self.grid.root().transpose()
    }
}

impl<G: BaseGrid> BaseGrid for Transpose<G> {
    type Item = G::Item;

    unsafe fn get_unchecked(&self, location: &Location) -> &Self::Item {
        self.grid.get_unchecked(&location.transpose())
    }
}

impl<G: BaseGridSetter> BaseGridSetter for Transpose<G> {
    unsafe fn set_unchecked(&mut self, location: &Location, value: Self::Item) {
        self.grid.set_unchecked(&location.transpose(), value)
    }
}

impl<G: BaseGridMut> BaseGridMut for Transpose<G> {
    unsafe fn get_unchecked_mut(&mut self, location: &Location) -> &mut Self::Item {
        self.grid.get_unchecked_mut(&location.transpose())
    }
}
