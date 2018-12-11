use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Sub, SubAssign};

use derive_more::*;

use crate::direction::*;
use crate::grid::GridBounds;
use crate::vector::{Columns, Component as VecComponent, Rows, Vector};

// TODO: add additional implied traits

/// A component of a [`Location`]
///
/// This trait comprises a component of a [`Location`], which may be either a
/// [`Row`] or a [`Column`]
pub trait Component: Sized + From<isize> + Into<isize> + Copy {
    /// The converse component ([`Row`] to [`Column`], or vice versa)
    type Converse: Component;

    /// The associated vector component ([`Rows`] or [`Columns`])
    type Distance: VecComponent;

    /// Get this component type from a [`Location`]
    fn from_location(location: &Location) -> Self;

    /// Combine this component with its converse to create a [`Location`]
    fn combine(self, other: Self::Converse) -> Location;

    /// Return the lowercase name of this component type– "row" or "column"
    fn name() -> &'static str;
}

// TODO: add docstrings to these. Perhaps refer back to Component
macro_rules! make_component {
    (
        $Name:ident,
        $Converse:ident,
        $Distance:ident,
        $from_loc:ident,
        ($self:ident, $other:ident) =>
        ($first:ident, $second:ident),
        $name:expr
    ) => {
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, From, Into)]
        #[repr(transparent)]
        pub struct $Name(pub isize);

        /// Adding a component to its converse (ie, a [`Row`] to a [`Column`])
        /// produces a [`Location`]
        impl Add<$Converse> for $Name {
            type Output = Location;

            fn add(self, rhs: $Converse) -> Location {
                self.combine(rhs)
            }
        }

        impl<T: Into<$Distance>> Add<T> for $Name {
            type Output = $Name;

            fn add(self, rhs: T) -> Self {
                $Name(self.0 + rhs.into().0)
            }
        }

        impl<T: Into<$Distance>> AddAssign<T> for $Name {
            fn add_assign(&mut self, rhs: T) {
                self.0 += rhs.into().0
            }
        }

        impl<T: Into<$Distance>> Sub<T> for $Name {
            type Output = $Name;

            fn sub(self, rhs: T) -> Self {
                $Name(self.0 - rhs.into().0)
            }
        }

        impl<T: Into<$Distance>> SubAssign<T> for $Name {
            fn sub_assign(&mut self, rhs: T) {
                self.0 -= rhs.into().0
            }
        }


        impl Component for $Name {
            type Converse = $Converse;
            type Distance = $Distance;

            #[inline]
            fn from_location(loc: &Location) -> Self {
                loc.$from_loc
            }

            fn combine($self, $other: Self::Converse) -> Location {
                Location::new($first, $second)
            }

            #[inline(always)]
            fn name() -> &'static str {
                $name
            }
        }
    }
}

make_component! {Row, Column, Rows, row, (self, other) => (self, other), "row"}
make_component! {Column, Row, Columns, column, (self, other) => (other, self), "column"}

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

    pub fn get_component<T: Component>(&self) -> T {
        T::from_location(self)
    }

    pub fn above(&self, distance: impl Into<Rows>) -> Location {
        *self - distance.into()
    }

    pub fn below(&self, distance: impl Into<Rows>) -> Location {
        *self + distance.into()
    }

    pub fn left(&self, distance: impl Into<Columns>) -> Location {
        *self - distance.into()
    }

    pub fn right(&self, distance: impl Into<Columns>) -> Location {
        *self + distance.into()
    }

    pub fn relative(&self, direction: Direction, distance: isize) -> Location {
        *self + Vector::in_direction(direction, distance)
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

/// A location that has passed a bounds check for a given grid. This struct
/// has no public constructors; it can only be created by the range checkers
/// of GridBounds.
///
/// TODO: confirm that the lifetime bounds are effective
/// TODO: integrate this struct into the Grid types
#[derive(Debug, Clone)]
pub struct CheckedLocation<'a, T: GridBounds> {
    location: Location,
    grid: PhantomData<&'a T>,
}
