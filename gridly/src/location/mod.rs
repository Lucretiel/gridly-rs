pub mod component;
mod range;

use core::cmp::Ordering;
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Deref, DerefMut, Sub, SubAssign};

use crate::direction::Direction;
use crate::vector::{Columns, Rows, Vector};

pub use component::{Column, Component, Row};
pub use range::Range;

//TODO: separate type for dimensions; essentially an unsigned Vector

/// A location on a grid
///
/// A location is the primary indexing type for a Grid, and represents a single
/// cell on that grid. It is comprised of a Row and a Column. By convention,
/// increasing rows count downward and increasing columns count rightward.
///
/// Locations support arithmetic operations with Vectors.
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct Location {
    pub row: Row,
    pub column: Column,
}

impl Location {
    pub fn new(row: impl Into<Row>, column: impl Into<Column>) -> Self {
        Location {
            row: row.into(),
            column: column.into(),
        }
    }

    /// Get either the row or column of a location
    pub fn get_component<T: Component>(&self) -> T {
        T::from_location(self)
    }

    /// Return the location that is `distance` above this one
    pub fn above(&self, distance: impl Into<Rows>) -> Location {
        *self - distance.into()
    }

    /// Return the location that is `distance` below this one
    pub fn below(&self, distance: impl Into<Rows>) -> Location {
        *self + distance.into()
    }

    /// Return the location that is `distance` to the left of this one
    pub fn left(&self, distance: impl Into<Columns>) -> Location {
        *self - distance.into()
    }

    /// Return the location that is `distance` to the right of this one
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
        self.into()
    }

    pub fn column_ordered(self) -> ColumnOrderedLocation {
        self.into()
    }
}

impl<R: Into<Row>, C: Into<Column>> From<(R, C)> for Location {
    fn from(value: (R, C)) -> Location {
        Location::new(value.0, value.1)
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

    fn as_tuple(&self) -> (M, M::Converse) {
        (self.get_component(), self.get_component())
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

impl<Major: Component> Ord for OrderedLocation<Major> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.as_tuple().cmp(&rhs.as_tuple())
    }
}

pub type RowOrderedLocation = OrderedLocation<Row>;
pub type ColumnOrderedLocation = OrderedLocation<Column>;
