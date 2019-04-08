use crate::grid::{BaseGrid, GridBounds};
use crate::location::Location;
use crate::vector::Vector;

pub trait IntoTranspose: Sized {
    fn transpose(self) -> Transpose<Self>;
}

impl<G: GridBounds + Sized> IntoTranspose for G {
    fn transpose(self) -> Transpose<Self> {
        Transpose { grid: self }
    }
}

/// Transpose a grid, swapping its rows and columns
#[derive(Debug, Clone, Default, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Transpose<G> {
    grid: G,
}

impl<G> Transpose<G> {
    pub fn transpose(self) -> G {
        self.grid
    }
}

impl<G: GridBounds> GridBounds for Transpose<G> {
    fn dimensions(&self) -> Vector {
        self.grid.dimensions().transpose()
    }

    fn root(&self) -> Location {
        self.grid.root().transpose()
    }
}

impl<G: BaseGrid> BaseGrid for Transpose<G> {
    type Item = G::Item;

    unsafe fn get_unchecked(&self, loc: &Location) -> &Self::Item {
        self.grid.get_unchecked(&loc.transpose())
    }
}
