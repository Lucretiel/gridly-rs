use std::collections::HashMap;
use std::iter::FusedIterator;

use gridly::prelude::*;

/// A sparse grid, where most of the cells are some default grid.
///
/// Sparse grids are backed by a hash table and a default value, and all elements
/// not present in the hash table are considered to be the default value. These
/// are colloquially called "unoccupied cells", though from the point of view
/// of the gridly interface, they are indistinguishable from other cells. When
/// reading from the grid, references to unoccupied cells will always be to the
/// same stored default value.
///
/// Whenever possible, cells that are set to the default value (as determined by
/// `PartialEq`) will be removed from the hash table. Conversely, getting a
/// mutable reference to an unnocupied cell will insert a clone of the default
/// at that location, which can then be mutated.
#[derive(Debug, Clone)]
pub struct SparseGrid<T: Clone + PartialEq> {
    root: Location,
    dimensions: Vector,
    default: T,
    storage: HashMap<Location, T>,
}

impl<T: Clone + PartialEq> SparseGrid<T> {
    /// Create a new `SparseGrid` with the given dimensions, rooted at `(0, 0)`,
    /// filled with the given default value
    pub fn new(dimensions: Vector, default: T) -> Self {
        Self::new_rooted(Location::zero(), dimensions, default)
    }

    /// Create a new `SparseGrid` with the given dimensions and root location,
    /// filled with the default value
    pub fn new_rooted(root: Location, dimensions: Vector, default: T) -> Self {
        Self {
            root,
            dimensions,
            default,
            storage: HashMap::new(),
        }
    }

    /// Get a reference to the default value. Most cells in the grid have this value.
    pub fn get_default(&self) -> &T {
        &self.default
    }

    /// Remove all entries from the underlying hash table that compare equal to
    /// the default
    pub fn clean(&mut self) {
        let default = &self.default;
        self.storage.retain(move |_, value| value != default)
    }

    /// Get an iterator over all of the occupied (non-default) entries in the
    /// grid, in an arbitrary order.
    pub fn occuppied_entries(
        &self,
    ) -> impl Iterator<Item = (&Location, &T)> + FusedIterator + Clone {
        let default = &self.default;
        self.storage
            .iter()
            .filter(move |(_, value)| *value != default)
    }

    /// Get an iterator of mutable references to the occupied (non-default)
    /// entries in the grid, in an arbitrary order.
    pub fn occuppied_entries_mut(
        &mut self,
    ) -> impl Iterator<Item = (&Location, &mut T)> + FusedIterator {
        let default = &self.default;
        self.storage
            .iter_mut()
            .filter(move |(_, value)| *value != default)
    }
}

impl<T: Clone + PartialEq + Default> SparseGrid<T> {
    /// Create a new `SparseGrid` with the given dimensions and root location,
    /// filled with the default value for `T`
    pub fn new_default_rooted(root: Location, dimensions: Vector) -> Self {
        Self::new_rooted(root, dimensions, T::default())
    }

    /// Create a new `SparseGrid` with the given dimensions, rooted at `(0, 0)`,
    /// filled with the default value for `T`
    pub fn new_default(dimensions: Vector) -> Self {
        Self::new(dimensions, T::default())
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

    /// Get a reference to a value in the grid. If the location is not present
    /// in the hash table, return a reference to the grid's default value.
    unsafe fn get_unchecked(&self, loc: &Location) -> &T {
        self.storage.get(loc).unwrap_or(&self.default)
    }
}

impl<T: Clone + PartialEq> BaseGridSetter for SparseGrid<T> {
    /// Set the value of a cell in the grid. If this value compares equal to
    /// the default, remove it from the underlying hash table.
    unsafe fn set_unchecked(&mut self, location: &Location, value: T) {
        if value == self.default {
            self.storage.remove(location);
        } else {
            self.storage.insert(*location, value);
        }
    }
}

impl<T: Clone + PartialEq> BaseGridMut for SparseGrid<T> {
    /// Get a mutable reference to a cell in the grid. If this cell is unoccupied,
    /// the default is cloned and inserted into the underlying hash table at this
    /// location.
    unsafe fn get_unchecked_mut(&mut self, location: &Location) -> &mut T {
        let default = &self.default;
        self.storage
            .entry(*location)
            .or_insert_with(move || default.clone())
    }
}
