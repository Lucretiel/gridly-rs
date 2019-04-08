mod component;

use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::direction::Direction;

pub use component::{Columns, Component, Rows};

// TODO: constify all of these methods

/// A measurement of distance between two [`Location`]s
///
/// A vector is the measurement of distance between two [`Location`]s. It
/// supports arithmetic operations with itself, as well as anything which can
/// be converted into a Vector. Currently, [`Rows`], [`Columns`], and [`Direction`]
/// all have this property, as well as a tuple of (Into<Rows>, Into<Columns>).
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct Vector {
    pub rows: Rows,
    pub columns: Columns,
}

impl Vector {
    /// Create a new `Vector`
    pub fn new(rows: impl Into<Rows>, columns: impl Into<Columns>) -> Self {
        Vector {
            rows: rows.into(),
            columns: columns.into(),
        }
    }

    /// Create a zero `Vector`
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

    /// Create a vector of the given size in the given direction
    pub fn in_direction(direction: Direction, length: isize) -> Vector {
        direction.sized_vec(length)
    }

    /// Return the Manhattan length of the vector
    ///
    /// The Manhattan length of a vector is the sum of the absolute values of
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
        Vector::new(-self.rows, -self.columns)
    }

    /// Generically get either the `Rows` or `Columns` of a vector
    pub fn get_component<T: Component>(&self) -> T {
        T::from_vector(self)
    }

    /// Swap the rows and columns of this Vector
    pub fn transpose(&self) -> Vector {
        Vector::new(self.columns.transpose(), self.rows.transpose())
    }
}

/// Convert a `Rows` or `Columns` into an equivelent Vector
impl<C: Component> From<C> for Vector {
    #[inline]
    fn from(distance: C) -> Self {
        distance.combine(Default::default())
    }
}

/// Convert a `Direction` into a unit vector pointing in that direction
impl From<Direction> for Vector {
    #[inline]
    fn from(direction: Direction) -> Self {
        direction.unit_vec()
    }
}

impl<R: Into<Rows>, C: Into<Columns>> From<(R, C)> for Vector {
    #[inline]
    fn from(value: (R, C)) -> Vector {
        Vector::new(value.0, value.1)
    }
}

impl<T: Into<Vector>> Add<T> for Vector {
    type Output = Vector;

    #[inline]
    fn add(self, rhs: T) -> Vector {
        let rhs = rhs.into();
        Vector::new(self.rows + rhs.rows, self.columns + rhs.columns)
    }
}

impl<T: Into<Vector>> AddAssign<T> for Vector {
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.rows += rhs.rows;
        self.columns += rhs.columns;
    }
}

impl<T: Into<Vector>> Sub<T> for Vector {
    type Output = Vector;

    #[inline]
    fn sub(self, rhs: T) -> Vector {
        let rhs = rhs.into();
        Vector::new(self.rows - rhs.rows, self.columns - rhs.columns)
    }
}

impl<T: Into<Vector>> SubAssign<T> for Vector {
    #[inline]
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.rows -= rhs.rows;
        self.columns -= rhs.columns;
    }
}

/// Multiply a vector's components by a constant factor
impl<T: Copy> Mul<T> for Vector
where
    isize: Mul<T, Output = isize>,
{
    type Output = Vector;

    #[inline]
    fn mul(self, factor: T) -> Vector {
        Vector::new(self.rows * factor, self.columns * factor)
    }
}

/// Multiply a vector's components by a constant factor in-place
impl<T: Copy> MulAssign<T> for Vector
where
    isize: MulAssign<T>,
{
    #[inline]
    fn mul_assign(&mut self, factor: T) {
        self.rows *= factor;
        self.columns *= factor;
    }
}

impl Neg for Vector {
    type Output = Vector;

    #[inline]
    fn neg(self) -> Vector {
        self.reverse()
    }
}
