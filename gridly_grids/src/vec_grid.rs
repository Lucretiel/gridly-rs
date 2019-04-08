use std::iter::repeat_with;

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
    fn prepare(
        dimensions: impl Into<Vector>,
        make_storage: impl FnOnce(usize, &Vector, &mut Vec<T>),
    ) -> Option<(Vector, Vec<T>)> {
        // TODO: should this function be unsafe?
        let dimensions = dimensions.into();

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

    /// Create a new `VecGrid`, filled with elements by repeatedly calling a function
    pub fn new_fill_with(dimensions: impl Into<Vector>, gen: impl FnMut() -> T) -> Option<Self> {
        Self::prepare(dimensions, move |volume, _dim, storage| {
            storage.extend(repeat_with(gen).take(volume));
        })
        .map(|(dimensions, storage)| Self {
            dimensions,
            storage,
        })
    }

    /// Create a new `VecGrid` by calling a function for each location in the
    /// grid
    pub fn new_with(
        dimensions: impl Into<Vector>,
        mut gen: impl FnMut(&Location) -> T,
    ) -> Option<Self> {
        Self::prepare(dimensions, move |_volume, dimensions, storage| {
            storage.extend(
                RowRange::range(dimensions.rows)
                    .flat_map(move |row| {
                        ColumnRange::range(dimensions.columns).map(move |column| row + column)
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
