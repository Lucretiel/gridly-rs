#![no_std]
//! Adapters for gridly grids. These adapters are designed to wrap other
//! to provide things like translation and transposition.

use gridly::prelude::*;

/// Grid adapter that translates the locations of the wrapped grid. The
/// translation is added to the inner grid location; that is, if the inner
/// grid has a root at `(0, 0)`, and the translation is `(2, 3)`, the new
/// root will be `(2, 3)`.
///
/// # Example
///
/// ```
/// use gridly_grids::VecGrid;
/// use gridly_adapters::Translate;
/// use gridly::prelude::*;
///
/// let grid: VecGrid<i32> = VecGrid::new_row_major(
///     Rows(2) + Columns(2),
///     [1, 2, 3, 4].iter().copied()
/// ).unwrap();
///
/// let grid = Translate::new(grid, Rows(2) + Columns(3));
///
/// assert_eq!(grid.get((2, 3)).ok(), Some(&1));
/// assert_eq!(grid.get((2, 4)).ok(), Some(&2));
/// assert_eq!(grid.get((3, 3)).ok(), Some(&3));
/// assert_eq!(grid.get((3, 4)).ok(), Some(&4));
///
/// assert_eq!(grid.get((0, 0)).ok(), None);
/// ```
#[derive(Debug, Clone)]
pub struct Translate<G> {
    grid: G,
    translation: Vector,
}

impl<G: GridBounds> Translate<G> {
    pub fn new(grid: G, translation: impl VectorLike) -> Self {
        Self {
            grid,
            translation: translation.as_vector(),
        }
    }
}

impl<G> Translate<G> {
    pub fn into_inner(self) -> G {
        self.grid
    }

    pub fn translation(&self) -> Vector {
        self.translation
    }
}

impl<G> AsRef<G> for Translate<G> {
    fn as_ref(&self) -> &G {
        &self.grid
    }
}

impl<G> AsMut<G> for Translate<G> {
    fn as_mut(&mut self) -> &mut G {
        &mut self.grid
    }
}

impl<G: GridBounds> GridBounds for Translate<G> {
    #[inline]
    fn dimensions(&self) -> Vector {
        self.grid.dimensions()
    }

    #[inline]
    fn root(&self) -> Location {
        self.grid.root() + self.translation
    }
}

impl<G: Grid> Grid for Translate<G> {
    type Item = G::Item;

    #[inline]
    unsafe fn get_unchecked(&self, location: Location) -> &Self::Item {
        self.grid.get_unchecked(location - self.translation)
    }
}

impl<G: GridMut> GridMut for Translate<G> {
    unsafe fn get_unchecked_mut(&mut self, location: Location) -> &mut Self::Item {
        self.grid.get_unchecked_mut(location - self.translation)
    }
}

impl<G: GridSetter> GridSetter for Translate<G> {
    unsafe fn replace_unchecked(&mut self, location: Location, value: Self::Item) -> Self::Item {
        self.grid
            .replace_unchecked(location - self.translation, value)
    }

    unsafe fn set_unchecked(&mut self, location: Location, value: Self::Item) {
        self.grid.set_unchecked(location - self.translation, value)
    }
}

/// Grid adapter that translates the locations of the wrapped grid such that
/// the root is always `(0, 0)`. Useful when combined with [`Window`].
///
/// # Example
///
/// ```
/// use gridly_grids::SparseGrid;
/// use gridly_adapters::ZeroRoot;
/// use gridly::prelude::*;
///
/// let mut grid: SparseGrid<i32> = SparseGrid::new_rooted(Row(1) + Column(1), Rows(3) + Columns(4));
/// grid.set(Row(1) + Column(2), 4).unwrap();
/// grid.set(Row(2) + Column(3), 5).unwrap();
///
/// let grid = ZeroRoot::new(grid);
///
/// assert_eq!(grid.get((0, 0)).ok(), Some(&0));
/// assert_eq!(grid.get((0, 1)).ok(), Some(&4));
/// assert_eq!(grid.get((1, 1)).ok(), Some(&0));
/// assert_eq!(grid.get((1, 2)).ok(), Some(&5));
///
/// assert_eq!(grid.get((3, 4)).ok(), None);
/// ```
#[derive(Debug, Clone)]
pub struct ZeroRoot<G> {
    grid: G,
}

impl<G: GridBounds> ZeroRoot<G> {
    fn offset_to_inner_root(&self) -> Vector {
        self.grid.root() - Location::zero()
    }

    pub fn new(grid: G) -> Self {
        Self { grid }
    }
}

impl<G> ZeroRoot<G> {
    pub fn into_inner(self) -> G {
        self.grid
    }
}

impl<G> AsRef<G> for ZeroRoot<G> {
    fn as_ref(&self) -> &G {
        &self.grid
    }
}

impl<G> AsMut<G> for ZeroRoot<G> {
    fn as_mut(&mut self) -> &mut G {
        &mut self.grid
    }
}

impl<G: GridBounds> GridBounds for ZeroRoot<G> {
    #[inline]
    fn dimensions(&self) -> Vector {
        self.grid.dimensions()
    }

    #[inline]
    fn root(&self) -> Location {
        Location::zero()
    }
}

impl<G: Grid> Grid for ZeroRoot<G> {
    type Item = G::Item;

    #[inline]
    unsafe fn get_unchecked(&self, location: Location) -> &Self::Item {
        self.grid
            .get_unchecked(location + self.offset_to_inner_root())
    }
}

impl<G: GridMut> GridMut for ZeroRoot<G> {
    unsafe fn get_unchecked_mut(&mut self, location: Location) -> &mut Self::Item {
        self.grid
            .get_unchecked_mut(location + self.offset_to_inner_root())
    }
}

impl<G: GridSetter> GridSetter for ZeroRoot<G> {
    unsafe fn replace_unchecked(&mut self, location: Location, value: Self::Item) -> Self::Item {
        self.grid
            .replace_unchecked(location + self.offset_to_inner_root(), value)
    }

    unsafe fn set_unchecked(&mut self, location: Location, value: Self::Item) {
        self.grid
            .set_unchecked(location + self.offset_to_inner_root(), value)
    }
}

/// Grid adapter that views a subset of the wrapped grid, using the same
/// coordinate system. For instance, given this grid:
///
/// ```text
/// OOOOO
/// OOOOO
/// OOOOO
/// OOOOO
/// ```
///
/// A window with a root of `(1, 1)` and dimensions of `(2, 2)` will cover this
/// region:
///
/// ```text
/// OOOOO
/// OXXOO
/// OXXOO
/// OOOOO
/// ```
///
/// # Example
///
/// ```
/// use gridly_grids::VecGrid;
/// use gridly_adapters::Window;
/// use gridly::prelude::*;
///
/// let grid: VecGrid<i32> = VecGrid::new_row_major(
///     Rows(4) + Columns(5),
///     1..
/// ).unwrap();
///
/// let grid = Window::new(
///     grid,
///     Row(1) + Column(1),
///     Rows(2) + Columns(2),
/// );
///
/// assert_eq!(grid.get((0, 0)).ok(), None);
/// assert_eq!(grid.get((1, 1)).ok(), Some(&7));
/// assert_eq!(grid.get((2, 2)).ok(), Some(&13));
/// assert_eq!(grid.get((3, 3)).ok(), None);
/// ```
#[derive(Debug, Clone)]
pub struct Window<G> {
    grid: G,
    root: Location,
    dimensions: Vector,
}

impl<G: GridBounds> Window<G> {
    pub fn new(grid: G, root: impl LocationLike, dimensions: impl VectorLike) -> Self {
        Self {
            grid,
            root: root.as_location(),
            dimensions: dimensions.as_vector(),
        }
    }
}
impl<G> Window<G> {
    pub fn into_inner(self) -> G {
        self.grid
    }
}

impl<G> AsRef<G> for Window<G> {
    fn as_ref(&self) -> &G {
        &self.grid
    }
}

impl<G> AsMut<G> for Window<G> {
    fn as_mut(&mut self) -> &mut G {
        &mut self.grid
    }
}

impl<G: GridBounds> GridBounds for Window<G> {
    fn dimensions(&self) -> Vector {
        // Vector from the base root to self.root
        let root_offset = self.root - self.grid.root();

        // Max size dimensions of a grid rooted at self.root
        let max_dimensions = self.grid.dimensions() - root_offset;

        Vector {
            rows: self.dimensions.rows.min(max_dimensions.rows),
            columns: self.dimensions.columns.min(max_dimensions.columns),
        }
    }

    fn root(&self) -> Location {
        let base_root = self.grid.root();
        let adjusted_root = self.root;

        Location {
            row: base_root.row.max(adjusted_root.row),
            column: base_root.column.max(adjusted_root.column),
        }
    }
}

impl<G: Grid> Grid for Window<G> {
    type Item = G::Item;

    unsafe fn get_unchecked(&self, location: Location) -> &Self::Item {
        self.grid.get_unchecked(location)
    }
}

impl<G: GridMut> GridMut for Window<G> {
    unsafe fn get_unchecked_mut(&mut self, location: Location) -> &mut Self::Item {
        self.grid.get_unchecked_mut(location)
    }
}

impl<G: GridSetter> GridSetter for Window<G> {
    unsafe fn replace_unchecked(&mut self, location: Location, value: Self::Item) -> Self::Item {
        self.grid.replace_unchecked(location, value)
    }

    unsafe fn set_unchecked(&mut self, location: Location, value: Self::Item) {
        self.grid.set_unchecked(location, value)
    }
}

/// Grid adapter that transposes the row & column of the wrapped grid. This is
/// a diagonal reflection through (0, 0).
#[derive(Debug, Clone)]
pub struct Transpose<G> {
    grid: G,
}

impl<G: GridBounds> Transpose<G> {
    pub fn new(grid: G) -> Self {
        Self { grid }
    }
}

impl<G: GridBounds> GridBounds for Transpose<G> {
    #[inline]
    fn dimensions(&self) -> Vector {
        self.grid.dimensions().transpose()
    }

    #[inline]
    fn root(&self) -> Location {
        self.grid.root().transpose()
    }
}

impl<G: Grid> Grid for Transpose<G> {
    type Item = G::Item;

    #[inline]
    unsafe fn get_unchecked(&self, location: Location) -> &Self::Item {
        self.grid.get_unchecked(location.transpose())
    }
}

impl<G: GridMut> GridMut for Transpose<G> {
    unsafe fn get_unchecked_mut(&mut self, location: Location) -> &mut Self::Item {
        self.grid.get_unchecked_mut(location.transpose())
    }
}

impl<G: GridSetter> GridSetter for Transpose<G> {
    unsafe fn replace_unchecked(&mut self, location: Location, value: Self::Item) -> Self::Item {
        self.grid.replace_unchecked(location.transpose(), value)
    }

    unsafe fn set_unchecked(&mut self, location: Location, value: Self::Item) {
        self.grid.set_unchecked(location.transpose(), value)
    }
}
