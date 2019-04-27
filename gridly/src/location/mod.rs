pub mod component;
mod range;

use core::cmp::{Ordering, PartialOrd};
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Deref, DerefMut, Sub, SubAssign};

use crate::direction::Direction;
use crate::vector::{Columns, Rows, Vector};

pub use component::{Column, Component, Row};
pub use range::Range;

/// A location on a grid
///
/// A location is the primary indexing type for a Grid, and represents a single
/// cell on that grid. It is comprised of a Row and a Column. Increasing rows
/// count downward and increasing columns count rightward.
///
/// Locations support arithmetic operations with Vectors.
#[derive(Debug, Clone, Copy, Default, Hash, Eq)]
pub struct Location {
    pub row: Row,
    pub column: Column,
}

impl Location {
    /// Create a new location out of a `row` and a `column`
    pub fn new(row: impl Into<Row>, column: impl Into<Column>) -> Self {
        Location {
            row: row.into(),
            column: column.into(),
        }
    }

    /// Create a new location at `(0, 0)`.
    pub const fn zero() -> Self {
        Location {
            row: Row(0),
            column: Column(0),
        }
    }

    /// Get either the row or column of a location
    pub fn get_component<T: Component>(&self) -> T {
        T::from_location(self)
    }

    /// Return the location that is `distance` rows above this one
    pub fn above(&self, distance: impl Into<Rows>) -> Location {
        *self - distance.into()
    }

    /// Return the location that is `distance` rows below this one
    pub fn below(&self, distance: impl Into<Rows>) -> Location {
        *self + distance.into()
    }

    /// Return the location that is `distance` columns to the left of this one
    pub fn left(&self, distance: impl Into<Columns>) -> Location {
        *self - distance.into()
    }

    /// Return the location that is `distance` columns to the right of this one
    pub fn right(&self, distance: impl Into<Columns>) -> Location {
        *self + distance.into()
    }

    /// Return the location that is `distance` away in the given `direction`
    pub fn relative(&self, direction: Direction, distance: isize) -> Location {
        *self + (direction * distance)
    }

    /// Return the location that is 1 away in the given `direction`
    pub fn step(&self, direction: Direction) -> Location {
        *self + direction
    }

    /// Swap the row and colimn of this Location
    pub fn transpose(&self) -> Location {
        Location::new(self.column.transpose(), self.row.transpose())
    }

    pub fn order_by<Major: Component>(self) -> OrderedLocation<Major> {
        self.into()
    }

    pub fn row_ordered(self) -> RowOrderedLocation {
        self.order_by()
    }

    pub fn column_ordered(self) -> ColumnOrderedLocation {
        self.order_by()
    }
}

impl From<(Row, Column)> for Location {
    fn from(value: (Row, Column)) -> Location {
        Location::new(value.0, value.1)
    }
}

impl From<(Column, Row)> for Location {
    fn from(value: (Column, Row)) -> Location {
        Location::new(value.1, value.0)
    }
}

impl<T: Into<Vector>> Add<T> for Location {
    type Output = Location;

    fn add(self, rhs: T) -> Location {
        let rhs = rhs.into();
        Location::new(self.row + rhs.rows, self.column + rhs.columns)
    }
}

impl<T: Into<Vector>> AddAssign<T> for Location {
    fn add_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.row += rhs.rows;
        self.column += rhs.columns;
    }
}

impl<T: Into<Vector>> Sub<T> for Location {
    type Output = Location;

    fn sub(self, rhs: T) -> Location {
        let rhs = rhs.into();
        Location::new(self.row - rhs.rows, self.column - rhs.columns)
    }
}

impl<T: Into<Vector>> SubAssign<T> for Location {
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.row -= rhs.rows;
        self.column -= rhs.columns;
    }
}

impl Sub<Location> for Location {
    type Output = Vector;

    fn sub(self, rhs: Location) -> Vector {
        Vector::new(self.row - rhs.row, self.column - rhs.column)
    }
}

impl<T: Into<Location> + Copy> PartialEq<T> for Location {
    fn eq(&self, rhs: &T) -> bool {
        let rhs = (*rhs).into();
        self.row == rhs.row && self.column == rhs.column
    }
}

/// Locations have a partial ordering. `loc1` is considered greater than `loc2` iff
/// its row or its column are greater than those in `loc2`. This chart shows an
/// example:
///
/// ```text
/// <<<??
/// <<<??
/// <<=>>
/// ??>>>
/// ??>>>
/// ```
///
/// Cells marked `>` are considered greater than the center location (marked `=`),
/// and cells marked '<' are less than the center location. Cells marked `?` do
/// not have an ordering with the center location.
///
/// For a strict ordering between all possible locations, see the [`OrderedLocation`]
/// wrapper struct, which allows for row-major or column-major orderings.
impl<T: Into<Location> + Copy> PartialOrd<T> for Location {
    fn partial_cmp(&self, rhs: &T) -> Option<Ordering> {
        let rhs = (*rhs).into();
        match (self.row.cmp(&rhs.row), self.column.cmp(&rhs.column)) {
            (Ordering::Greater, Ordering::Less) | (Ordering::Less, Ordering::Greater) => None,
            (o1, o2) => Some(o1.then(o2)),
        }
    }
}

#[cfg(test)]
mod partial_ord_tests {
    use crate::prelude::{Columns, Location, Rows, Vector};
    use crate::shorthand::{R, C, L};
    use core::cmp::Ordering;

    const ZERO: Location = Location::zero();
    const DIAG: Vector = Vector {
        rows: Rows(1),
        columns: Columns(1),
    };
    const BAD_DIAG: Vector = Vector {
        rows: Rows(-1),
        columns: Columns(1),
    };

    #[test]
    fn test_eq() {
        assert_eq!(ZERO, (R(0), C(0)));
        assert_eq!(ZERO, L(0, 0));
    }

    #[test]
    fn test_orderliness() {
        assert_eq!(ZERO.partial_cmp(&L(-1, 0)), Some(Ordering::Greater));
        assert_eq!(ZERO.partial_cmp(&L(0, -1)), Some(Ordering::Greater));
        assert_eq!(ZERO.partial_cmp(&(R(-1), C(-1))), Some(Ordering::Greater));

        assert_eq!(ZERO.partial_cmp(&ZERO.below(1)), Some(Ordering::Less));
        assert_eq!(ZERO.partial_cmp(&ZERO.right(1)), Some(Ordering::Less));
        assert_eq!(ZERO.partial_cmp(&(ZERO + DIAG)), Some(Ordering::Less));

        assert_eq!(ZERO.partial_cmp(&(ZERO + BAD_DIAG)), None);
        assert_eq!(ZERO.partial_cmp(&(ZERO - BAD_DIAG)), None);

        assert_eq!(ZERO.partial_cmp(&ZERO), Some(Ordering::Equal));
    }

    #[test]
    fn test_bad_diagonal() {
        for location in &[L(1, -1), L(-1, 1)] {
            assert!(!(ZERO < *location));
            assert!(!(ZERO > *location));
            assert!(!(ZERO <= *location));
            assert!(!(ZERO >= *location));
        }
    }
}

/// Rules for ordering a Location
///
/// `OrderedLocation` is a wrapper struct around a `Location` that supplies an Ord and
/// PartialOrd implementation. The `Major` type parameter indicates which
/// ordering is used; for instance, `Ordering<Row>` provides row-major ordering,
/// where Locations are sorted first by row, then by column.
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct OrderedLocation<Major: Component> {
    pub location: Location,
    phantom: PhantomData<Major>,
}

impl<M: Component> OrderedLocation<M> {
    pub fn new(location: Location) -> Self {
        Self {
            location,
            phantom: PhantomData,
        }
    }
}

impl<M: Component> From<OrderedLocation<M>> for Location {
    fn from(ord: OrderedLocation<M>) -> Self {
        ord.location
    }
}

impl<M: Component> From<Location> for OrderedLocation<M> {
    fn from(location: Location) -> Self {
        Self::new(location)
    }
}

impl<M: Component> AsRef<Location> for OrderedLocation<M> {
    fn as_ref(&self) -> &Location {
        &self.location
    }
}

impl<M: Component> AsMut<Location> for OrderedLocation<M> {
    fn as_mut(&mut self) -> &mut Location {
        &mut self.location
    }
}

impl<M: Component> Deref for OrderedLocation<M> {
    type Target = Location;

    fn deref(&self) -> &Location {
        &self.location
    }
}

impl<M: Component> DerefMut for OrderedLocation<M> {
    fn deref_mut(&mut self) -> &mut Location {
        &mut self.location
    }
}

impl<M: Component> PartialOrd for OrderedLocation<M> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }

    fn lt(&self, rhs: &Self) -> bool {
        self.cmp(rhs) == Ordering::Less
    }

    fn le(&self, rhs: &Self) -> bool {
        self.cmp(rhs) != Ordering::Greater
    }

    fn gt(&self, rhs: &Self) -> bool {
        self.cmp(rhs) == Ordering::Greater
    }

    fn ge(&self, rhs: &Self) -> bool {
        self.cmp(rhs) != Ordering::Less
    }
}

impl<M: Component> Ord for OrderedLocation<M> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        M::from_location(self)
            .cmp(&M::from_location(rhs))
            .then_with(move || {
                M::Converse::from_location(self).cmp(&M::Converse::from_location(rhs))
            })
    }
}

pub type RowOrderedLocation = OrderedLocation<Row>;
pub type ColumnOrderedLocation = OrderedLocation<Column>;
