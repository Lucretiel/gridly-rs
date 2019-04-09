use std::collections::HashMap;
use std::iter::FusedIterator;

use gridly::prelude::*;

#[derive(Debug, Clone)]
pub struct SparseGrid<T: Clone + PartialEq> {
    root: Location,
    dimensions: Vector,
    default: T,
    storage: HashMap<Location, T>,
}

impl<T: Clone + PartialEq> SparseGrid<T> {
    pub fn new(dimensions: Vector, default: T) -> Self {
        Self::new_rooted(Location::zero(), dimensions, default)
    }

    pub fn new_rooted(root: Location, dimensions: Vector, default: T) -> Self {
        Self {
            root,
            dimensions,
            default,
            storage: HashMap::new(),
        }
    }

    pub fn get_default(&self) -> &T {
        &self.default
    }

    /// Remove all entries from the grid that compare equal to the default
    pub fn clean(&mut self) {
        let default = &self.default;
        self.storage.retain(move |_, value| value != default)
    }

    /// Get an iterator over all of the occupied entries in the grid, in an arbitrary order.
    pub fn occuppied_entries(
        &self,
    ) -> impl Iterator<Item = (&Location, &T)> + FusedIterator + Clone {
        let default = &self.default;
        self.storage
            .iter()
            .filter(move |(_, value)| *value != default)
    }

    pub fn occuppied_entries_mut(
        &mut self,
    ) -> impl Iterator<Item = (&Location, &mut T)> + FusedIterator {
        let default = &self.default;
        self.storage
            .iter_mut()
            .filter(move |(_, value)| *value != default)
    }
}

impl<T: Clone + PartialEq> BaseGridBounds for SparseGrid<T> {
    fn dimensions(&self) -> Vector {
        self.dimensions
    }

    fn root(&self) -> Location {
        self.root
    }
}

impl<T: Clone + PartialEq> BaseGrid for SparseGrid<T> {
    type Item = T;

    unsafe fn get_unchecked(&self, loc: &Location) -> &T {
        self.storage.get(loc).unwrap_or(&self.default)
    }
}

impl<T: Clone + PartialEq> BaseGridSetter for SparseGrid<T> {
    unsafe fn set_unchecked(&mut self, location: &Location, value: T) {
        if value == self.default {
            self.storage.remove(location);
        } else {
            self.storage.insert(*location, value);
        }
    }
}
