use crate::grid::{BaseGrid, BaseGridBounds, BaseGridSetter, Grid, GridSetter};
use crate::location::Location;
use crate::vector::Vector;

pub trait IntoBordered: Sized + BaseGrid {
    fn with_border(self, border_value: Self::Item) -> Bordered<Self>;
}

impl<G: BaseGrid + Sized> IntoBordered for G {
    fn with_border(self, border_value: Self::Item) -> Bordered<Self> {
        Bordered {
            border_value,
            grid: self,
        }
    }
}

#[derive(Debug, Clone, Default, Hash, Eq, PartialEq)]
pub struct Bordered<G: BaseGrid> {
    border_value: G::Item,
    grid: G,
}

impl<G: BaseGrid> Bordered<G> {
    pub fn inner_grid(&self) -> &G {
        &self.grid
    }

    pub fn inner_grid_mut(&mut self) -> &mut G {
        &mut self.grid
    }

    pub fn inner_root(&self) -> Location {
        self.grid.root()
    }

    pub fn inner_dimensions(&self) -> Vector {
        self.grid.dimensions()
    }

    pub fn border_value(&self) -> &G::Item {
        &self.border_value
    }
}

impl<G: BaseGrid> BaseGridBounds for Bordered<G> {
    fn root(&self) -> Location {
        self.grid.root() - (1, 1)
    }

    fn dimensions(&self) -> Vector {
        self.grid.dimensions() + (2, 2)
    }
}

impl<G: BaseGrid> BaseGrid for Bordered<G> {
    type Item = G::Item;

    unsafe fn get_unchecked(&self, location: &Location) -> &Self::Item {
        self.grid.get(*location).unwrap_or(&self.border_value)
    }
}

pub enum BorderCollision<E> {
    Collision,
    Interior(E),
}

impl<G: BaseGridSetter> BaseGridSetter for Bordered<G> {
    type SetError = BorderCollision<G::SetError>;

    unsafe fn try_set_unchecked(
        &mut self,
        location: &Location,
        value: Self::Item,
    ) -> Result<(), Self::SetError> {
        self.grid
            .try_set(*location, value)
            .map_err(|_err| BorderCollision::Collision)?
            .map_err(|err| BorderCollision::Interior(err))
    }
}
