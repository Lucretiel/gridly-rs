use std::iter::repeat_with;
use std::ops::Index;

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
    /// This function pre-allocates an empty vector of the correct size, determined
    /// by `dimensions`. It then calls `make_storage` with the volume of the grid,
    /// the dimensions of the grid, and the empty vector. `make_storage` is expected
    /// to fill the vector with elements based on the particular constructor being
    /// used.
    ///
    /// Returns the dimensions of the grid and the storage vector, or `None` if
    /// the dimensions are invalid.
    fn prepare(
        dimensions: Vector,
        make_storage: impl FnOnce(usize, &Vector, &mut Vec<T>),
    ) -> Option<(Vector, Vec<T>)> {
        // TODO: should this function be unsafe?

        if dimensions.rows < 0 || dimensions.columns < 0 {
            None
        } else {
            (dimensions.rows.0 as usize)
                .checked_mul(dimensions.columns.0 as usize)
                .map(move |volume| {
                    let mut storage = Vec::with_capacity(volume);
                    make_storage(volume, &dimensions, &mut storage);
                    (dimensions, storage)
                })
        }
    }

    /// Create a new `VecGrid`, filled with elements by repeatedly calling a
    /// function. The function is called once per cell in an unspecified order;
    /// use [`new_with`] if you want to have per-cell initialization logic.
    ///
    /// Returns the grid, or `None` if the `dimensions` were invalid.
    ///
    /// # Example:
    ///
    /// ```
    /// use gridly_grids::VecGrid;
    /// use gridly::prelude::*;
    /// use gridly::shorthand::L;
    ///
    /// let grid = VecGrid::new_fill_with(Rows(2) + Columns(2), || "Hello, World!".to_string()).unwrap();
    /// assert_eq!(grid[L(1, 0)], "Hello, World!")
    /// ```
    ///
    /// See also [`new`] for filling a grid with a type's [default] value, and
    /// [`new_fill`] for filling a grid with a clone of a value.
    pub fn new_fill_with(dimensions: impl Into<Vector>, gen: impl FnMut() -> T) -> Option<Self> {
        Self::prepare(dimensions.into(), move |volume, _dim, storage| {
            storage.extend(repeat_with(gen).take(volume));
        })
        .map(|(dimensions, storage)| Self {
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
    pub fn new_with(
        dimensions: impl Into<Vector>,
        mut gen: impl FnMut(&Location) -> T,
    ) -> Option<Self> {
        Self::prepare(dimensions.into(), move |_volume, dimensions, storage| {
            storage.extend(
                RowRange::span(Row(0), dimensions.rows)
                    .flat_map(move |row| {
                        ColumnRange::span(Column(0), dimensions.columns)
                            .map(move |column| row + column)
                    })
                    .map(move |location| gen(&location)),
            );
        })
        .map(|(dimensions, storage)| Self {
            dimensions,
            storage,
        })
    }

    /// Given a bounds-checked location, return the index of the vector
    /// associated with that location. Performs no bounds checking, either
    /// on the input `location` or the output `usize`
    unsafe fn index_for_location(&self, loc: &Location) -> usize {
        (loc.row.0 as usize * self.dimensions.columns.0 as usize) + loc.column.0 as usize
    }
}

impl<T: Default> VecGrid<T> {
    /// Create a new `VecGrid` filled with the default value of `T` in each cell
    pub fn new(dimensions: impl Into<Vector>) -> Option<Self> {
        Self::new_fill_with(dimensions, Default::default)
    }
}

impl<T: Clone> VecGrid<T> {
    /// Create a new `VecGrid` filled with clones of `value`
    pub fn new_fill(dimensions: impl Into<Vector>, value: &T) -> Option<Self> {
        Self::new_fill_with(dimensions, move || value.clone())
    }
}

impl<T> BaseGridBounds for VecGrid<T> {
    fn dimensions(&self) -> Vector {
        self.dimensions
    }
}

impl<T> BaseGrid for VecGrid<T> {
    type Item = T;

    unsafe fn get_unchecked(&self, location: Location) -> &T {
        self.storage
            .get_unchecked(self.index_for_location(&location))
    }
}

impl<T> Index<Location> for VecGrid<T> {
    type Output = T;

    fn index(&self, location: Location) -> &T {
        self.get(location).unwrap_or_else(|bounds_err| {
            panic!("{:?} out of bounds: {}", location, bounds_err)
        })
    }
}

impl<T> BaseGridSetter for VecGrid<T> {
    unsafe fn set_unchecked(&mut self, location: Location, value: T) {
        let index = self.index_for_location(&location);
        *self.storage.get_unchecked_mut(index) = value;
    }
}

impl<T> BaseGridMut for VecGrid<T> {
    unsafe fn get_unchecked_mut(&mut self, location: Location) -> &mut T {
        let index = self.index_for_location(&location);
        self.storage.get_unchecked_mut(index)
    }
}
