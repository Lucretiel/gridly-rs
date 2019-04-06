pub mod component;
mod range;

use core::ops::{Add, AddAssign, Sub, SubAssign};

pub use component::{Column, Component, Row};
pub use range::Range;

use crate::direction::Direction;
use crate::vector::{Columns, Rows, Vector};

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
