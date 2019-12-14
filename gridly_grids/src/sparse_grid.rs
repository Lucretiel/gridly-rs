use std::collections::HashMap;
use std::fmt::Debug;
use std::iter::FusedIterator;
use std::ops::{Index, IndexMut};

use gridly::prelude::*;

/// A sparse grid, where most of the cells are some default grid.
///
/// Sparse grids are backed by a hash table and a default value, and all elements
/// not present in the hash table are considered to be the default value. These
/// are colloquially called "unoccupied cells", though from the point of view
/// of the gridly interface, they are indistinguishable from other cells. When
/// reading from the grid, references to unoccupied cells will (usually) be to
/// the same stored default value.
///
/// Whenever possible, cells that are set to the default value (as determined by
/// `PartialEq`) will be removed from the hash table. Conversely, getting a
/// mutable reference to an unnocupied cell will insert a clone of the default
/// at that location, which can then be mutated.
///
/// Note about interior mutability: When a user gets a reference to an unoccupied
/// cell, the reference will (usually) be to the stored default value. This means
/// that if the user mutates the cell somehow (for instance, with a `RefCell`),
/// those changes will appear in all unoccupied cells.
///
/// This is a trivial implementation of a Sparse Grid, intended for simple use
/// cases and as an example `Grid` implementation. More complex implementations
/// are possible that track dirtied cells and clear them from the internal
/// storage more aggressively.
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
    pub fn new_default(dimensions: impl VectorLike, default: T) -> Self {
        Self::new_rooted_default(Location::zero(), dimensions, default)
    }

    /// Create a new `SparseGrid` with the given dimensions and root location,
    /// filled with the default value
    pub fn new_rooted_default(
        root: impl LocationLike,
        dimensions: impl VectorLike,
        default: T,
    ) -> Self {
        Self {
            root: root.as_location(),
            dimensions: dimensions.as_vector(),
            default,
            storage: HashMap::new(),
        }
    }

    /// Get a reference to the default value.
    pub fn get_default(&self) -> &T {
        &self.default
    }

    /// Remove all entries from the underlying hash table that compare equal to
    /// the default
    pub fn clean(&mut self) {
        let default = &self.default;
        self.storage.retain(move |_, value| value != default);
    }

    /// Remove all non-default entries from the grid
    pub fn clear(&mut self) {
        self.storage.clear();
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
            .filter(move |(_, value)| value != &default)
    }

    /// Get an iterator of mutable references to the occupied (non-default)
    /// entries in the grid, in an arbitrary order.
    ///
    /// The difference between this method and `occuppied_entries_mut` is that
    /// this one first [cleans](SparseGrid::clean) the underlying storage.
    /// This means there's a higher up-front cost, but has the benefit of
    /// providing an `ExactSizeIterator`.
    pub fn occuppied_entries_mut_cleaned(
        &mut self,
    ) -> impl Iterator<Item = (&Location, &mut T)> + FusedIterator + ExactSizeIterator {
        self.clean();
        self.storage.iter_mut()
    }

    /// Insert a value into this grid at an arbitrary location. If the location
    /// is outside the grid's bounds, the grid's bounds are updated to include
    /// this value. Returns the previous value.
    ///
    /// # Example
    ///
    /// ```
    /// use gridly_grids::SparseGrid;
    /// use gridly::prelude::*;
    ///
    /// let mut grid: SparseGrid<isize> = SparseGrid::new((0, 0));
    ///
    /// assert_eq!(grid.root(), (0, 0));
    /// assert_eq!(grid.dimensions(), (0, 0));
    ///
    /// grid.insert((-1, 0), 10);
    ///
    /// assert_eq!(grid.root(), (-1, 0));
    /// assert_eq!(grid.dimensions(), (1, 1));
    /// assert_eq!(grid[(-1, 0)], 10);
    ///
    /// grid.insert((0, 1), 5);
    ///
    /// assert_eq!(grid.root(), (-1, 0));
    /// assert_eq!(grid.dimensions(), (2, 2));
    /// assert_eq!(grid[(0, 1)], 5);
    ///
    /// grid.insert((1, 0), 4);
    ///
    /// assert_eq!(grid.root(), (-1, 0));
    /// assert_eq!(grid.dimensions(), (3, 2));
    /// assert_eq!(grid[(1, 0)], 4);
    ///
    /// grid.insert((0, -1), 3);
    ///
    /// assert_eq!(grid.root(), (-1, -1));
    /// assert_eq!(grid.dimensions(), (3, 3));
    /// assert_eq!(grid[(0, -1)], 3);
    /// ```
    #[inline]
    pub fn insert(&mut self, location: impl LocationLike, value: T) -> T {
        let location = location.as_location();

        let outer_row = self.root.row + self.dimensions.rows;
        let outer_column = self.root.column + self.dimensions.columns;

        if location.row < self.root.row {
            let diff = self.root.row - location.row;
            self.root.row = location.row;
            self.dimensions.rows += diff;
        } else if location.row >= outer_row {
            self.dimensions.rows = (location.row - self.root.row) + 1;
        }

        if location.column < self.root.column {
            let diff = self.root.column - location.column;
            self.root.column = location.column;
            self.dimensions.columns += diff;
        } else if location.column >= outer_column {
            self.dimensions.columns = (location.column - self.root.column) + 1;
        }

        // Safety: not really unsafe, because HashMap has no unsafe accessors.
        // However, we're assured that the dimensions are correct after the
        // above logic.
        unsafe { self.replace_unchecked(&location, value) }
    }
}

impl<T: Clone + PartialEq + Default> SparseGrid<T> {
    /// Create a new `SparseGrid` with the given dimensions and root location,
    /// filled with the default value for `T`
    pub fn new_rooted(root: impl LocationLike, dimensions: impl VectorLike) -> Self {
        Self::new_rooted_default(root, dimensions, T::default())
    }

    /// Create a new `SparseGrid` with the given dimensions, rooted at `(0, 0)`,
    /// filled with the default value for `T`
    pub fn new(dimensions: impl VectorLike) -> Self {
        Self::new_default(dimensions, T::default())
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

impl<T: Clone + PartialEq> Grid for SparseGrid<T> {
    type Item = T;

    /// Get a reference to a value in the grid. If the location is not present
    /// in the hash table, return a reference to the grid's default value.
    unsafe fn get_unchecked(&self, loc: &Location) -> &T {
        self.storage.get(loc).unwrap_or(&self.default)
    }
}

impl<T: Clone + PartialEq, L: LocationLike> Index<L> for SparseGrid<T> {
    type Output = T;

    fn index(&self, location: L) -> &T {
        self.get(&location).unwrap_or_else(|bounds_err| {
            panic!("{:?} out of bounds: {}", location.as_location(), bounds_err)
        })
    }
}

impl<T: Clone + PartialEq> GridSetter for SparseGrid<T> {
    /// Set the value of a cell in the grid. If this value compares equal to
    /// the default, remove it from the underlying hash table. Return the
    /// previous value (which may be a clone of the default value if the cell
    /// was unoccupied)
    unsafe fn replace_unchecked(&mut self, location: &Location, value: Self::Item) -> Self::Item {
        if value == self.default {
            self.storage.remove(location).unwrap_or(value)
        } else {
            self.storage
                .insert(*location, value)
                .unwrap_or_else(move || self.default.clone())
        }
    }

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

impl<T: Clone + PartialEq> GridMut for SparseGrid<T> {
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

impl<T: Clone + PartialEq, L: LocationLike> IndexMut<L> for SparseGrid<T> {
    fn index_mut(&mut self, location: L) -> &mut T {
        self.get_mut(&location).unwrap_or_else(|bounds_err| {
            panic!("{:?} out of bounds: {}", location.as_location(), bounds_err)
        })
    }
}
