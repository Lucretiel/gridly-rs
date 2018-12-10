use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

use derive_more::*;

use crate::location::{Component as LocComponent, Row, Column};
use crate::direction::*;

/// A component of a [`Vector`]
///
/// This trait comprises a component of a [`Vector`], which may be either a row
/// or a column. This trait is provided as an aid to code that is generic over
/// Rows or Columns.
pub trait Component: Sized + From<isize> + Into<isize> {
    /// The converse component ([`Rows`] to [`Columns`] or vice versa)
    type Converse: Component;

    /// The assoicated location component type
    type Point: LocComponent;

    /// Get this compnent from a [`Vector`]
    fn from_vector(vector: &Vector) -> Self;

    /// Create a vector from a Row and Column
    fn combine(self, other: Self::Converse) -> Vector;
}

macro_rules! make_component {
    (
        $Name:ident,
        $Converse:ident,
        $Point:ident,
        $from_vec:ident,
        ($self:ident, $other:ident) =>
        ($first:ident, $second:ident)
    ) => {
        #[derive(
            Debug,
            Clone,
            Copy,
            Default,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            Add,
            AddAssign,
            Sub,
            SubAssign,
            Mul,
            MulAssign,
            Neg,
            From,
            Into,
        )]
        #[repr(transparent)]
        pub struct $Name(pub isize);

        impl Add<$Converse> for $Name {
            type Output = Vector;

            fn add(self, rhs: $Converse) -> Vector {
                self.combine(rhs)
            }
        }

        impl Component for $Name {
            type Converse = $Converse;
            type Point = $Point;

            fn from_vector(vector: &Vector) -> Self {
                vector.$from_vec
            }

            fn combine($self, $other: $Converse) -> Vector {
                Vector::new($first, $second)
            }
        }
    }
}

make_component!{Rows, Columns, Row, rows, (self, other) => (self, other)}
make_component!{Columns, Rows, Column, columns, (self, other) => (other, self)}

/// A measurement of distance between two [`Location`]s
///
/// A vector is the measurement of distance between two [`Location`]s. It
/// supports arithmetic operations with itself, and also with [`Direction`]s
/// (which are considered to be length 1 vectors in the given direction)
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    Hash,
    PartialEq,
    Eq,
    Mul,
    MulAssign,
    Neg,
)]
pub struct Vector {
    pub rows: Rows,
    pub columns: Columns,
}

impl Vector {
    /// Create a new [Vector]
    pub fn new(rows: impl Into<Rows>, columns: impl Into<Columns>) -> Self {
        Vector {
            rows: rows.into(),
            columns: columns.into(),
        }
    }

    /// Create a vector from a pair of components
    pub fn from_components<T: Component>(a: T, b: T::Converse) -> Self {
        a.combine(b)
    }

    pub fn zero() -> Vector {
        Vector::new(0, 0)
    }

    /// Create an upward pointing vector of the given size
    pub fn upward(size: isize) -> Vector {
        Vector::new(-size, 0)
    }

    /// Create a downward pointing vector of the given size
    pub fn downward(size: isize) -> Vector {
        Vector::new(size, 0)
    }

    /// Create a leftward pointing vector of the given size
    pub fn leftward(size: isize) -> Vector {
        Vector::new(0, -size)
    }

    /// Create a rightward pointing vector of the given size
    pub fn rightward(size: isize) -> Vector {
        Vector::new(0, size)
    }

    pub fn in_direction(direction: Direction, distance: isize) -> Vector {
        direction.unit_vec() * distance
    }

    /// Return the Manhattan length of the vector
    ///
    /// The Manhattan length of a vector is the some of the absolute values of
    /// its components
    pub fn manhattan_length(&self) -> isize {
        self.rows.0.abs() + self.columns.0.abs()
    }

    /// Return a new vector, rotated 90 degrees clockwise.
    pub fn clockwise(&self) -> Vector {
        // (-1, 0) -> (0, 1) -> (1, 0) -> (0, -1)
        Vector::new(self.columns.0, -self.rows.0)
    }

    /// Return a new vector, rotated 90 degrees counterclockwise.
    pub fn counterclockwise(&self) -> Vector {
        Vector::new(-self.columns.0, self.rows.0)
    }

    // Return a new vector, facing the opposite direction of this one
    pub fn reverse(&self) -> Vector {
        Vector {
            rows: -self.rows,
            columns: -self.columns,
        }
    }

    pub fn get_component<T: Component>(&self) -> T {
        T::from_vector(self)
    }
}

impl From<Rows> for Vector {
    fn from(rows: Rows) -> Self {
        Vector::new(rows, 0)
    }
}

impl From<Columns> for Vector {
    fn from(columns: Columns) -> Self {
        Vector::new(0, columns)
    }
}

impl<R: Into<Rows>, C: Into<Columns>> From<(R, C)> for Vector {
    fn from(value: (R, C)) -> Vector {
        Vector::new(value.0, value.1)
    }
}

impl<T: Into<Vector>> Add<T> for Vector {
    type Output = Vector;

    fn add(self, rhs: T) -> Vector {
        let rhs = rhs.into();
        Vector::new(self.rows + rhs.rows, self.columns + rhs.columns)
    }
}

impl<T: Into<Vector>> AddAssign<T> for Vector {
    fn add_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.rows += rhs.rows;
        self.columns += rhs.columns;
    }
}

impl<T: Into<Vector>> Sub<T> for Vector {
    type Output = Vector;

    fn sub(self, rhs: T) -> Vector {
        let rhs = rhs.into();
        Vector::new(self.rows - rhs.rows, self.columns - rhs.columns)
    }
}

impl<T: Into<Vector>> SubAssign<T> for Vector {
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.rows -= rhs.rows;
        self.columns -= rhs.columns;
    }
}