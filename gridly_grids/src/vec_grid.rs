use std::iter::repeat_with;
use std::mem::replace;
use std::ops::{Index, IndexMut};

use gridly::prelude::*;

/// A grid that stores its elements in a `Vec<T>`, in row-major order.
#[derive(Debug, Clone)]
pub struct VecGrid<T> {
    dimensions: Vector,
    storage: Vec<T>,
}

impl<T> VecGrid<T> {
    /// Given the prospective dimensions of a grid, return the volume of the
    /// grid if the dimensions are valid, or None otherwise. Used as a helper
    /// in the `VecGrid` constructors.
    fn get_volume(dimensions: &Vector) -> Option<usize> {
        if dimensions.rows < 0 || dimensions.columns < 0 {
            None
        } else {
            (dimensions.rows.0 as usize).checked_mul(dimensions.columns.0 as usize)
        }
    }

    /// Given a bounds-checked location, return the index of the vector
    /// associated with that location. Performs no bounds checking, either
    /// on the input `location` or the output `usize`
    unsafe fn index_for_location(&self, loc: &Location) -> usize {
        (loc.row.0 as usize * self.dimensions.columns.0 as usize) + loc.column.0 as usize
    }

    /// Create a new `VecGrid`, filled with elements by repeatedly calling a
    /// function. The function is called once per cell in an unspecified order;
    /// use [`new_with`][VecGrid::new_with] if you want to have per-cell
    /// initialization logic.
    ///
    /// Returns the grid, or `None` if the `dimensions` were invalid.
    ///
    /// # Example:
    ///
    /// ```
    /// use gridly_grids::VecGrid;
    /// use gridly::prelude::*;
    ///
    /// let grid = VecGrid::new_fill_with((Rows(2), Columns(2)), || "Hello, World!".to_string()).unwrap();
    /// assert_eq!(grid[(1, 0)], "Hello, World!")
    /// ```
    ///
    /// See also [`new`][VecGrid::new] for filling a grid with a type's
    /// [default][Default] value, and [`new_fill`][VecGrid::new_fill] for
    /// filling a grid with a clone of a value.
    pub fn new_fill_with(dimensions: impl VectorLike, gen: impl Fn() -> T) -> Option<Self> {
        let dimensions = dimensions.as_vector();
        let volume = Self::get_volume(&dimensions)?;
        let storage = repeat_with(gen).take(volume).collect();
        Some(VecGrid {
            dimensions,
            storage,
        })
    }

    /// Create a new `VecGrid` by calling a function with the location of each cell
    /// in the grid, storing the return value of that function in that cell.
    ///
    /// The function is called once per cell in an unspecified order; users should
    /// not rely on it being called in row-major order.
    ///
    /// Returns the grid, or `None` if the `dimensions` were invalid.
    ///
    /// # Example:
    ///
    /// ```
    /// use gridly_grids::VecGrid;
    /// use gridly::prelude::*;
    ///
    /// let grid = VecGrid::new_with((Rows(2), Columns(2)), |loc| loc.row.0 + loc.column.0).unwrap();
    /// assert_eq!(grid.get((0, 0)), Ok(&0));
    /// assert_eq!(grid.get((0, 1)), Ok(&1));
    /// assert_eq!(grid.get((1, 0)), Ok(&1));
    /// assert_eq!(grid.get((1, 1)), Ok(&2));
    /// assert!(grid.get((1, 2)).is_err());
    /// ```
    pub fn new_with(
        dimensions: impl VectorLike,
        mut gen: impl FnMut(&Location) -> T,
    ) -> Option<Self> {
        let dimensions = dimensions.as_vector();
        let columns = dimensions.columns;

        let mut storage = Vec::with_capacity(Self::get_volume(&dimensions)?);

        storage.extend(
            Row(0)
                .span(dimensions.rows)
                .flat_map(move |row| Column(0).span(columns).cross(row))
                .map(move |loc| gen(&loc)),
        );

        Some(VecGrid {
            dimensions,
            storage,
        })
    }

    /// Create a new `VecGrid` with the given dimensions. Fills the grid
    /// in row-major order (that is, by filling the first row, then the next
    /// row, etc) by evaluating the input iterator. Returns None if
    /// the iterator was too short, or if the dimensions were invalid. If the
    /// iterator is longer than required, the remaining elements are left
    /// uniterated.
    ///
    /// # Example:
    ///
    /// ```
    /// use gridly_grids::VecGrid;
    /// use gridly::prelude::*;
    ///
    /// let data = [1, 2, 3, 4, 5];
    /// let mut data_iter = data[..].iter().copied();
    ///
    /// let grid = VecGrid::new_row_major(
    ///     (Rows(2), Columns(2)),
    ///     &mut data_iter
    /// ).unwrap();
    ///
    /// assert_eq!(grid[(0, 0)], 1);
    /// assert_eq!(grid[(0, 1)], 2);
    /// assert_eq!(grid[(1, 0)], 3);
    /// assert_eq!(grid[(1, 1)], 4);
    ///
    /// // excess elements in the iterator are still available
    /// assert_eq!(data_iter.next(), Some(5));
    /// assert_eq!(data_iter.next(), None);
    /// ```
    pub fn new_row_major(
        dimensions: impl VectorLike,
        input: impl IntoIterator<Item = T>,
    ) -> Option<Self> {
        let dimensions = dimensions.as_vector();
        let volume = Self::get_volume(&dimensions)?;

        // We could just input.collect(), but the iterator may have a size_hint
        // lower bound of 0, so we want to make sure we preallocate. Arguably,
        // we should let the Vec manage its own storage, because a size_hint
        // lower bound of zero means that it may not finish, and we don't want
        // to overallocate unnecessary. However, we assume the fast path is
        // that it usually will finish.
        let mut storage = Vec::with_capacity(volume);
        storage.extend(input.into_iter().take(volume));

        if storage.len() != volume {
            None
        } else {
            Some(VecGrid {
                dimensions,
                storage,
            })
        }
    }

    /// Fill every cell in the grid with the values produced by repeatedly
    /// calling `gen`. Called in an unspecified order.
    ///
    /// # Example
    ///
    /// ```
    /// use gridly_grids::VecGrid;
    /// use gridly::prelude::*;
    ///
    /// let mut grid: VecGrid<isize> = VecGrid::new((Rows(2), Columns(2))).unwrap();
    ///
    /// grid.fill_with(|| 3);
    /// assert_eq!(grid.get((0, 0)), Ok(&3));
    /// assert_eq!(grid.get((0, 1)), Ok(&3));
    /// assert_eq!(grid.get((1, 0)), Ok(&3));
    /// assert_eq!(grid.get((1, 1)), Ok(&3));
    /// assert!(grid.get((1, 2)).is_err());
    /// ```
    pub fn fill_with(&mut self, gen: impl Fn() -> T) {
        // TODO: right now we're assuming it's faster to run all the destructors
        // at once, then rebuild the vector from scratch.
        let volume = self.storage.len();
        self.storage.clear();
        self.storage.extend(repeat_with(gen).take(volume))
    }

    /// Fill the grid in row-major order with values from an iterator.
    /// That is, fill the first row, then the next row, etc.
    ///
    /// If the iterator is longer than the volume of the grid, the
    /// remaining elements are left uniterated. If the iterator is shorter
    /// than the volume of the grid, the remaining existing elements in the
    /// grid are left unaltered.
    ///
    /// # Example
    ///
    /// ```
    /// use gridly_grids::VecGrid;
    /// use gridly::prelude::*;
    ///
    /// let mut grid = VecGrid::new_fill((Rows(2), Columns(2)), &10).unwrap();
    /// let values = [1, 2, 3];
    /// let values_iter = values[..].iter().copied();
    ///
    /// grid.fill_row_major(values_iter);
    ///
    /// assert_eq!(grid[(0, 0)], 1);
    /// assert_eq!(grid[(0, 1)], 2);
    /// assert_eq!(grid[(1, 0)], 3);
    /// assert_eq!(grid[(1, 1)], 10);
    /// ```
    pub fn fill_row_major(&mut self, input: impl IntoIterator<Item = T>) {
        // TODO: a naked for loop may be more performant, since Zip doesn't
        // have fold or try_fold. For now we do it the functional way and wait
        // for zip to gain a try fold specialization.
        let volume = self.storage.len();
        input
            .into_iter()
            .take(volume)
            .zip(&mut self.storage)
            .for_each(|(item, cell)| *cell = item);
    }
}

impl<T: Default> VecGrid<T> {
    /// Create a new `VecGrid` filled with the default value of `T` in each cell
    ///
    /// # Example
    ///
    /// ```
    /// use gridly_grids::VecGrid;
    /// use gridly::prelude::*;
    ///
    /// let grid: VecGrid<isize> = VecGrid::new((Rows(2), Columns(2))).unwrap();
    /// assert_eq!(grid.get((0, 0)), Ok(&0));
    /// assert_eq!(grid.get((0, 1)), Ok(&0));
    /// assert_eq!(grid.get((1, 0)), Ok(&0));
    /// assert_eq!(grid.get((1, 1)), Ok(&0));
    /// assert!(grid.get((1, 2)).is_err());
    /// ```
    pub fn new(dimensions: impl VectorLike) -> Option<Self> {
        Self::new_fill_with(dimensions, Default::default)
    }

    /// Create a new `VecGrid` with the given dimensions. Fills the grid
    /// in row-major order (that is, by filling the first row, then the next
    /// row, etc) by evaluating the input iterator. Returns None if the
    /// dimensions were invalid. If the iterator is shorter than required to
    /// fill the grid, the remaining elements are filled with `T::default`.
    /// iterator is longer than required, the remaining elements are left
    /// uniterated.
    ///
    /// # Example:
    ///
    /// ```
    /// use gridly_grids::VecGrid;
    /// use gridly::prelude::*;
    ///
    /// let data = [1, 2, 3];
    /// let data_iter = data[..].iter().copied();
    ///
    /// let grid = VecGrid::new_row_major_default(
    ///     (Rows(2), Columns(2)),
    ///     data_iter
    /// ).unwrap();
    ///
    /// assert_eq!(grid[(0, 0)], 1);
    /// assert_eq!(grid[(0, 1)], 2);
    /// assert_eq!(grid[(1, 0)], 3);
    /// assert_eq!(grid[(1, 1)], 0);
    /// ```
    pub fn new_row_major_default(
        dimensions: impl VectorLike,
        input: impl IntoIterator<Item = T>,
    ) -> Option<Self> {
        Self::new_row_major(dimensions, input.into_iter().chain(repeat_with(T::default)))
    }

    /// Replace all the cells in the grid with the default value
    ///
    /// # Example
    ///
    /// ```
    /// use gridly_grids::VecGrid;
    /// use gridly::prelude::*;
    ///
    /// let mut grid = VecGrid::new_fill((Rows(2), Columns(2)), &5).unwrap();
    /// grid.clear();
    /// assert_eq!(grid.get((0, 0)), Ok(&0));
    /// assert_eq!(grid.get((0, 1)), Ok(&0));
    /// assert_eq!(grid.get((1, 0)), Ok(&0));
    /// assert_eq!(grid.get((1, 1)), Ok(&0));
    /// assert!(grid.get((1, 2)).is_err());
    /// ```
    pub fn clear(&mut self) {
        self.fill_with(T::default);
    }
}

impl<T: Clone> VecGrid<T> {
    /// Create a new `VecGrid` filled with clones of `value`
    ///
    /// # Example
    ///
    /// ```
    /// use gridly_grids::VecGrid;
    /// use gridly::prelude::*;
    ///
    /// let grid = VecGrid::new_fill((Rows(2), Columns(2)), &"Hello").unwrap();
    /// assert_eq!(grid.get((0, 0)), Ok(&"Hello"));
    /// assert_eq!(grid.get((0, 1)), Ok(&"Hello"));
    /// assert_eq!(grid.get((1, 0)), Ok(&"Hello"));
    /// assert_eq!(grid.get((1, 1)), Ok(&"Hello"));
    /// assert!(grid.get((1, 2)).is_err());
    /// ```
    pub fn new_fill(dimensions: impl VectorLike, value: &T) -> Option<Self> {
        Self::new_fill_with(dimensions, move || value.clone())
    }

    /// Fill every element in the grid with clones of `value`
    ///
    /// # Example
    ///
    /// ```
    /// use gridly_grids::VecGrid;
    /// use gridly::prelude::*;
    ///
    /// let mut grid = VecGrid::new((Rows(2), Columns(2))).unwrap();
    ///
    /// grid.fill(&"Hello");
    /// assert_eq!(grid.get((0, 0)), Ok(&"Hello"));
    /// assert_eq!(grid.get((0, 1)), Ok(&"Hello"));
    /// assert_eq!(grid.get((1, 0)), Ok(&"Hello"));
    /// assert_eq!(grid.get((1, 1)), Ok(&"Hello"));
    /// assert!(grid.get((1, 2)).is_err());
    /// ```
    pub fn fill(&mut self, value: &T) {
        self.fill_with(move || value.clone())
    }
}

impl<T: Copy> VecGrid<T> {
    /// Create a new `VecGrid` filled with copies of `value`
    ///
    /// # Example
    ///
    /// ```
    /// use gridly_grids::VecGrid;
    /// use gridly::prelude::*;
    ///
    /// let grid = VecGrid::new_fill_copied((Rows(2), Columns(2)), 10).unwrap();
    /// assert_eq!(grid.get((0, 0)), Ok(&10));
    /// assert_eq!(grid.get((0, 1)), Ok(&10));
    /// assert_eq!(grid.get((1, 0)), Ok(&10));
    /// assert_eq!(grid.get((1, 1)), Ok(&10));
    /// assert!(grid.get((1, 2)).is_err());
    /// ```
    pub fn new_fill_copied(dimensions: impl VectorLike, value: T) -> Option<Self> {
        Self::new_fill_with(dimensions, move || value)
    }
}

impl<T> GridBounds for VecGrid<T> {
    #[inline]
    fn dimensions(&self) -> Vector {
        self.dimensions
    }

    #[inline(always)]
    fn root(&self) -> Location {
        Location::zero()
    }
}

impl<T> Grid for VecGrid<T> {
    type Item = T;

    unsafe fn get_unchecked(&self, location: &Location) -> &T {
        self.storage
            .get_unchecked(self.index_for_location(location))
    }
}

impl<T, L: LocationLike> Index<L> for VecGrid<T> {
    type Output = T;

    fn index(&self, location: L) -> &T {
        self.get(&location).unwrap_or_else(|bounds_err| {
            panic!("{:?} out of bounds: {}", location.as_location(), bounds_err)
        })
    }
}

impl<T, L: LocationLike> IndexMut<L> for VecGrid<T> {
    fn index_mut(&mut self, location: L) -> &mut T {
        self.get_mut(&location).unwrap_or_else(|bounds_err| {
            panic!("{:?} out of bounds: {}", location.as_location(), bounds_err)
        })
    }
}

impl<T> GridSetter for VecGrid<T> {
    unsafe fn replace_unchecked(&mut self, location: &Location, value: T) -> T {
        let index = self.index_for_location(location);
        replace(self.storage.get_unchecked_mut(index), value)
    }

    unsafe fn set_unchecked(&mut self, location: &Location, value: T) {
        let index = self.index_for_location(location);
        *self.storage.get_unchecked_mut(index) = value;
    }
}

impl<T> GridMut for VecGrid<T> {
    unsafe fn get_unchecked_mut(&mut self, location: &Location) -> &mut T {
        let index = self.index_for_location(location);
        self.storage.get_unchecked_mut(index)
    }
}
