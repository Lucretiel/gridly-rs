use std::collections::HashMap;
use std::iter::{repeat, repeat_with, FusedIterator};

use gridly::prelude::*;

/// A grid that stores its elements in a `Vec<T>`, in row-major order.
#[derive(Debug, Clone)]
pub struct VecGrid<T> {
    dimensions: Vector,
    storage: Vec<T>,
}

impl<T> VecGrid<T> {
    /// Create a grid with an empty storage, preallocated. Ensures that the dimensions
    /// are positive, and that the total volume of this grid fits in a usize.
    ///
    /// This function returns the dimensions and vector because it doesn't do any
    /// bounds checking on the vector after make_storage, which means that the returned
    /// vector is not guaranteed to obey the invariants of the Grid
    fn build(dimensions: impl Into<Vector>, make_storage: impl FnOnce(usize, &Vector, &mut Vec<T>)) -> Option<(Vector, Vec<T>)> {
        // TODO: should this function be unsafe?
        let dimensions = dimensions.into();

        if dimensions.rows < 0 || dimensions.columns < 0 {
            None
        } else {
            (dimensions.rows.0 as usize).checked_mul(dimensions.columns.0 as usize)
                .map(move |volume| {
                    let mut storage = Vec::with_capacity(volume);
                    make_storage(volume, &dimensions, &mut storage);
                    (dimensions, storage)
                })
        }
    }

    pub fn new_fill_with(dimensions: impl Into<Vector>, gen: impl FnMut() -> T) -> Option<Self> {
        Self::build(dimensions, move |volume, _dim, storage| {
            storage.extend(repeat_with(gen).take(volume));
        }).map(|(dimensions, storage)| Self { dimensions, storage })
    }

    pub fn new_with(dimensions: impl Into<Vector>, mut gen: impl FnMut(&Location) -> T) -> Option<Self> {
        Self::build(dimensions, move |_volume, dimensions, storage| {
            storage.extend(RowRange::range(dimensions.rows)
                .flat_map(move |row| {
                    ColumnRange::range(dimensions.columns).map(move |column| row + column)
                })
                .map(move |location| gen(&location)))
        }).map(|(dimensions, storage)| Self { dimensions, storage })
    }
    /// Given a bounds-checked location, return the index of the vector
    /// associated with that location
    fn index_for_location(&self, loc: &Location) -> usize {
        (loc.row.0 as usize * self.dimensions.columns.0 as usize) + loc.column.0 as usize
    }
}

impl<T: Default> VecGrid<T> {
    pub fn new(dimensions: impl Into<Vector>) -> Option<Self> {
        Self::new_fill_with(dimensions, Default::default)
    }
}

impl<T: Clone> VecGrid<T> {
    pub fn new_fill(dimensions: impl Into<Vector>, value: &T) -> Option<Self> {
        Self::new_fill_with(dimensions, move || value.clone())
    }
}

impl<T> GridBounds for VecGrid<T> {
    fn dimensions(&self) -> Vector {
        self.dimensions
    }
}

impl<T> BaseGrid for VecGrid<T> {
    type Item = T;

    unsafe fn get_unchecked(&self, loc: &Location) -> &T {
        self.storage.get_unchecked(self.index_for_location(loc))
    }
}

impl<T> BaseGridMut for VecGrid<T> {
    unsafe fn get_unchecked_mut(&mut self, loc: &Location) -> &mut T {
        let index = self.index_for_location(loc);
        self.storage.get_unchecked_mut(index)
    }
}

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

impl<T: Clone + PartialEq> GridBounds for SparseGrid<T> {
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

impl<T: Clone + PartialEq> BaseGridMut for SparseGrid<T> {
    unsafe fn get_unchecked_mut(&mut self, loc: &Location) -> &mut T {
        let default = &self.default;
        self.storage
            .entry(*loc)
            .or_insert_with(move || default.clone())
    }

    unsafe fn set_unchecked(&mut self, loc: &Location, value: T) {
        if value == self.default {
            self.storage.remove(loc);
        } else {
            self.storage.insert(*loc, value);
        }
    }
}
