//! 2-dimensional [`Vector`] type used in [`Location`] arithmetic, with supporting types
//! and traits. A [`Vector`] is a measurement of distance between two [`Location`]s.
//!
//! [`Location`]: crate::location::Location
//! [`Vector`]: crate::vector::Vector

use core::cmp::Ordering;
use core::fmt::Debug;
use core::iter::Sum;
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::direction::*;
use crate::location::{Column, Component as LocComponent, Row};

/// A [`Rows`] or [`Columns`] component of a [`Vector`]
///
/// This trait comprises a component of a [`Vector`], which may be either
/// [`Rows`] or [`Columns`]. It represents a distance in a single direction,
/// either vertical ([`Rows`]) or horizontal ([`Columns`]).
///
/// In practice, most code will call methods directly on [`Rows`] or [`Columns`]
/// values. However, a lot of gridly functionality is agnostic towards
/// rows and columns (for instance, a view over a row in a grid is functionally
/// the same as a view over a column), so the `Component` trait allows such
/// functionality to be written generically.
///
/// The key methods for `Component` that allow it to work in generic contexts
/// are `from_vector`, which gets a component from a `Vector`, and `combine`,
/// which combines a [`Rows`] or [`Columns`] with its converse (a `Columns`
/// or a `Row`s) to create a new `Vector`.
pub trait Component:
    Sized
    + From<isize>
    + Copy
    + Ord
    + Eq
    + Debug
    + Add
    + AddAssign
    + Sub
    + SubAssign
    + Neg
    + Default
    + PartialEq<isize>
    + PartialOrd<isize>
    + VectorLike
    + Sum
{
    /// The converse component ([`Rows`] to [`Columns`] or vice versa)
    type Converse: Component<Converse = Self>;

    /// The assoicated location component type ([`Row`] or [`Column`])
    type Point: LocComponent<Distance = Self>;

    /// Get this compnent from a [`Vector`]
    ///
    /// # Example:
    /// ```
    /// use gridly::vector::*;
    ///
    /// let vec = Vector::new(4, 5);
    ///
    /// assert_eq!(Rows::from_vector(&vec), Rows(4));
    /// assert_eq!(Columns::from_vector(&vec), Columns(5));
    /// ```
    fn from_vector(vector: impl VectorLike) -> Self;

    /// Create a vector from a Row and Column
    ///
    /// # Example:
    ///
    /// ```
    /// use gridly::vector::*;
    ///
    /// let columns = Columns(10);
    /// let rows = Rows(2);
    ///
    /// assert_eq!(columns.combine(rows), Vector::new(2, 10));
    fn combine(self, converse: Self::Converse) -> Vector;

    /// Get the integer value of this component.
    ///
    /// # Example:
    ///
    /// ```
    /// use gridly::vector::*;
    ///
    /// let columns = Columns(10);
    /// let rows = Rows(2);
    ///
    /// assert_eq!(columns.value(), 10);
    /// assert_eq!(rows.value(), 2);
    /// ```
    fn value(self) -> isize;

    // Convert a Row into a Column or vice versa.
    //
    // # Example:
    ///
    /// ```
    /// use gridly::vector::*;
    ///
    /// assert_eq!(Rows(10).transpose(), Columns(10));
    /// ```
    fn transpose(self) -> Self::Converse;
}

// TODO: add docstrings to these. Perhaps refer back to Component
macro_rules! make_component {
    (
        $Name:ident,
        $Converse:ident,
        $Point:ident,
        $lower_name:ident,
        $lower_converse:ident,
        $name:literal,
        $test:ident
    ) => {
        #[doc="A "]
        #[doc=$name]
        #[doc=" component of a [`Vector`]"]
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
        )]
        #[repr(transparent)]
        pub struct $Name(pub isize);

        /// Adding a [`Rows`] to a [`Columns`] produces a [`Vector`]
        impl Add<$Converse> for $Name {
            type Output = Vector;

            #[inline]
            fn add(self, rhs: $Converse) -> Vector {
                self.combine(rhs)
            }
        }

        impl<T: Into<$Name>> Add<T> for $Name {
            type Output = Self;

            #[inline]
            fn add(self, rhs: T) -> Self {
                $Name(self.0 + rhs.into().0)
            }
        }

        impl<T: Into<$Name>> AddAssign<T> for $Name {
            #[inline]
            fn add_assign(&mut self, rhs: T) {
                self.0 += rhs.into().0
            }
        }

        impl<T: Into<$Name>> Sub<T> for $Name {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: T) -> Self {
                $Name(self.0 - rhs.into().0)
            }
        }

        impl<T: Into<$Name>> SubAssign<T> for $Name {
            #[inline]
            fn sub_assign(&mut self, rhs: T) {
                self.0 -= rhs.into().0
            }
        }

        impl<T> Mul<T> for $Name
            where isize: Mul<T, Output=isize>
        {
            type Output = Self;

            #[inline]
            fn mul(self, factor: T) -> Self {
                $Name(self.0 * factor)
            }
        }

        impl<T> MulAssign<T> for $Name
            where isize: MulAssign<T>
        {
            #[inline]
            fn mul_assign(&mut self, factor: T) {
                self.0 *= factor
            }
        }

        impl Neg for $Name {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self {
                $Name(-self.0)
            }
        }

        impl From<isize> for $Name {
            #[inline]
            fn from(value: isize) -> Self {
                $Name(value)
            }
        }

        impl PartialEq<isize> for $Name {
            #[inline]
            fn eq(&self, rhs: &isize) -> bool {
                self.0 == *rhs
            }
        }

        impl PartialOrd<isize> for $Name {
            #[inline]
            fn partial_cmp(&self, rhs: &isize) -> Option<Ordering> {
                self.0.partial_cmp(rhs)
            }

            #[inline]
            fn lt(&self, rhs: &isize) -> bool { self.0 < *rhs }
            #[inline]
            fn le(&self, rhs: &isize) -> bool { self.0 <= *rhs }
            #[inline]
            fn ge(&self, rhs: &isize) -> bool { self.0 >= *rhs }
            #[inline]
            fn gt(&self, rhs: &isize) -> bool { self.0 > *rhs }
        }

        /// A [`Rows`] or a [`Columns`] value can be treated as a [`Vector`]
        /// where the converse component is 0.
        impl VectorLike for $Name {
            #[inline]
            fn $lower_name(&self) -> $Name {
                *self
            }

            #[inline(always)]
            fn $lower_converse(&self) -> $Converse {
                $Converse(0)
            }

            #[inline]
            fn as_vector(&self) -> Vector {
                self.combine($Converse(0))
            }

            // TODO: custom implementations for VectorLike methods?
            // We assume that
        }

        impl VectorLike for ($Name, $Converse) {
            #[inline]
            fn $lower_name(&self) -> $Name {
                self.0
            }

            #[inline]
            fn $lower_converse(&self) -> $Converse {
                self.1
            }

            #[inline]
            fn as_vector(&self) -> Vector {
                self.0.combine(self.1)
            }

            // TODO: custom method implementations where they can be more
            // efficient. For now we assume that the compiler is smart enough
            // to inline (self.combine(0))
        }

        impl Sum for $Name {
            fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
                // TODO: is it better to do iter.map(value.0).sum()?
                iter.fold($Name(0), Add::add)
            }
        }

        impl<'a> Sum<&'a $Name> for $Name {
            fn sum<I: Iterator<Item=&'a Self>>(iter: I) -> Self {
                iter.fold($Name(0), |lhs, rhs| lhs + *rhs)
            }

        }

        impl Component for $Name {
            type Converse = $Converse;
            type Point = $Point;

            #[inline]
            fn from_vector(vector: impl VectorLike) -> Self {
                vector.$lower_name()
            }

            #[inline]
            fn combine(self, other: $Converse) -> Vector {
                Vector {
                    $lower_name: self,
                    $lower_converse: other,
                }
            }

            #[inline]
            fn value(self) -> isize {
                self.0
            }

            #[inline]
            fn transpose(self) -> Self::Converse {
                $Converse(self.0)
            }
        }

        #[cfg(test)]
        mod $test {
            use $crate::vector::{$Name, $Converse, Component, Vector};

            #[test]
            fn test_combine_converse() {
                let value = $Name(5);
                let converse = $Converse(8);

                assert_eq!(value.combine(converse), Vector {
                    $lower_name: value,
                    $lower_converse: converse,
                });
            }

            #[test]
            fn test_add_converse() {
                let value = $Name(5);
                let converse = $Converse(8);

                assert_eq!(value + converse, Vector {
                    $lower_name: value,
                    $lower_converse: converse,
                });
            }

            #[test]
            fn test_add() {
                let value = $Name(5);

                assert_eq!(value + $Name(5), $Name(10));
                assert_eq!(value + 5, $Name(10));
            }

            #[test]
            fn test_add_assign() {
                let mut value = $Name(5);

                value += $Name(5);
                assert_eq!(value, $Name(10));
                value += 5;
                assert_eq!(value, $Name(15));
            }

            #[test]
            fn test_sub() {
                let value = $Name(5);

                assert_eq!(value - $Name(5), $Name(0));
                assert_eq!(value - 5, $Name(0));
            }

            #[test]
            fn test_sub_assign() {
                let mut value = $Name(5);

                value -= $Name(5);
                assert_eq!(value, $Name(0));
                value -= 5;
                assert_eq!(value, $Name(-5));
            }

            #[test]
            fn test_mul() {
                assert_eq!($Name(5) * 4, $Name(20));
            }

            #[test]
            fn test_neg() {
                assert_eq!(-$Name(5), $Name(-5));
            }
        }
    }
}

make_component! {Rows, Columns, Row, rows, columns, "rows", test_rows}
make_component! {Columns, Rows, Column, columns, rows, "columns", test_columns}

// TODO: constify all of these methods

/// A measurement of distance between two [`Location`]s
///
/// A `Vector` is the measurement of distance between two
/// [`Location`]s. It supports arithmetic operations with itself, as well
/// as anything which can be converted into a Vector. Currently, [`Rows`],
/// [`Columns`], and [`Direction`] all have this property, as well as
/// `(Rows, Columns)`, `(Columns, Rows)`, and `(isize, isize)`
/// (which is treated as `(Rows, Columns)`).
///
/// [`Location`]: crate::location::Location
#[derive(Debug, Clone, Copy, Default, Hash, Eq)]
pub struct Vector {
    pub rows: Rows,
    pub columns: Columns,
}

impl Vector {
    /// Function for creating a new const vector. This is private for now; we're
    /// going to transition `new` to const once const methods in traits become
    /// available.
    const fn new_const(rows: isize, columns: isize) -> Self {
        Vector {
            rows: Rows(rows),
            columns: Columns(columns),
        }
    }

    /// Create a new `Vector`
    pub fn new(rows: impl Into<Rows>, columns: impl Into<Columns>) -> Self {
        Vector {
            rows: rows.into(),
            columns: columns.into(),
        }
    }

    /// Create a zero `Vector`
    pub const fn zero() -> Vector {
        Vector::new_const(0, 0)
    }

    /// Create an upward pointing vector of the given size
    ///
    /// ```
    /// use gridly::prelude::*;
    ///
    /// assert_eq!(Vector::upward(5), Vector::new(-5, 0))
    /// ```
    pub const fn upward(size: isize) -> Vector {
        Vector::new_const(-size, 0)
    }

    /// Create a downward pointing vector of the given size
    ///
    /// ```
    /// use gridly::prelude::*;
    ///
    /// assert_eq!(Vector::downward(5), Vector::new(5, 0))
    /// ```
    pub const fn downward(size: isize) -> Vector {
        Vector::new_const(size, 0)
    }

    /// Create a leftward pointing vector of the given size
    ///
    /// ```
    /// use gridly::prelude::*;
    ///
    /// assert_eq!(Vector::leftward(5), Vector::new(0, -5))
    /// ```
    pub const fn leftward(size: isize) -> Vector {
        Vector::new_const(0, -size)
    }

    /// Create a rightward pointing vector of the given size
    ///
    /// ```
    /// use gridly::prelude::*;
    ///
    /// assert_eq!(Vector::rightward(5), Vector::new(0, 5))
    /// ```
    pub const fn rightward(size: isize) -> Vector {
        Vector::new_const(0, size)
    }

    /// Create a vector of the given size in the given direction
    ///
    /// ```
    /// use gridly::prelude::*;
    ///
    /// assert_eq!(Vector::in_direction(Up, 10), Vector::new(-10, 0));
    /// ```
    pub fn in_direction(direction: Direction, length: isize) -> Vector {
        direction.sized_vec(length)
    }
}

///
pub trait VectorLike: Sized {
    fn rows(&self) -> Rows;
    fn columns(&self) -> Columns;
    fn as_vector(&self) -> Vector;

    /// Return the manhattan length of the vector. The manhattan length
    /// of a vector is the sum of the absolute values of its components.
    fn manhattan_length(&self) -> isize {
        self.rows().0.abs() + self.columns().0.abs()
    }

    /// Return the manhatten length of the vector, or `None` if there are
    /// any overflows.
    fn checked_manhattan_length(&self) -> Option<isize> {
        let rows = self.rows().0.checked_abs()?;
        let columns = self.columns().0.checked_abs()?;
        rows.checked_add(columns)
    }

    /// Return a new vector, rotated 90 degrees clockwise.
    fn clockwise(&self) -> Vector {
        // (-1, 0) -> (0, 1) -> (1, 0) -> (0, -1)
        Vector {
            rows: self.columns().transpose(),
            columns: -self.rows().transpose(),
        }
    }

    /// Return a new vector, rotated 90 degrees counterclockwise.
    fn anticlockwise(&self) -> Vector {
        Vector {
            rows: -self.columns().transpose(),
            columns: self.rows().transpose(),
        }
    }

    // Return a new vector, facing the opposite direction of this one
    fn reverse(&self) -> Vector {
        Vector {
            rows: -self.rows(),
            columns: -self.columns(),
        }
    }

    /// Generically get either the `Rows` or `Columns` of a vector
    fn get_component<T: Component>(&self) -> T {
        T::from_vector(self)
    }

    /// Swap the rows and columns of this Vector
    fn transpose(&self) -> Vector {
        Vector {
            rows: self.columns().transpose(),
            columns: self.rows().transpose(),
        }
    }

    /// If the vector is pointing in an orthogonal direction, return
    /// that direction
    fn direction(&self) -> Option<Direction> {
        let vec = self.as_vector();
        match (vec.rows.0, vec.columns.0) {
            (r, 0) if r < 0 => Some(Up),
            (r, 0) if r > 0 => Some(Down),
            (0, c) if c < 0 => Some(Left),
            (0, c) if c > 0 => Some(Right),
            _ => None,
        }
    }
}

impl VectorLike for Vector {
    #[inline]
    fn rows(&self) -> Rows {
        self.rows
    }

    #[inline]
    fn columns(&self) -> Columns {
        self.columns
    }

    #[inline]
    fn as_vector(&self) -> Vector {
        *self
    }
}

impl VectorLike for (isize, isize) {
    #[inline]
    fn rows(&self) -> Rows {
        Rows(self.0)
    }

    #[inline]
    fn columns(&self) -> Columns {
        Columns(self.1)
    }

    #[inline]
    fn as_vector(&self) -> Vector {
        Vector::new(self.0, self.1)
    }
}

impl<T: VectorLike> VectorLike for &T {
    #[inline]
    fn rows(&self) -> Rows {
        T::rows(self)
    }

    #[inline]
    fn columns(&self) -> Columns {
        T::columns(self)
    }

    #[inline]
    fn as_vector(&self) -> Vector {
        T::as_vector(self)
    }
}

impl<T: VectorLike> Add<T> for Vector {
    type Output = Vector;

    #[inline]
    fn add(self, rhs: T) -> Vector {
        let rhs = rhs.as_vector();
        Vector {
            rows: self.rows + rhs.rows,
            columns: self.columns + rhs.columns,
        }
    }
}

/// Test that Vectors can be added to themseleves, as well as all the things that
/// can be implicitly converted into Vectors
#[test]
fn test_add() {
    use crate::direction::*;

    let base = Vector::new(3, 4);
    assert_eq!(base + Vector::new(2, 3), (5, 7));

    assert_eq!(base + (Rows(1), Columns(2)), (4, 6));
    assert_eq!(base + (Columns(1), Rows(2)), (5, 5));

    assert_eq!(base + (1, 2), (4, 6));

    assert_eq!(base + Rows(5), (8, 4));
    assert_eq!(base + Columns(5), (3, 9));

    assert_eq!(base + Up, (2, 4));
    assert_eq!(base + Down, (4, 4));
    assert_eq!(base + Left, (3, 3));
    assert_eq!(base + Right, (3, 5));
}

impl<T: VectorLike> AddAssign<T> for Vector {
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        let rhs = rhs.as_vector();

        self.rows += rhs.rows;
        self.columns += rhs.columns;
    }
}

#[test]
fn test_add_assign() {
    use crate::direction::*;

    let mut base = Vector::zero();

    base += Vector::new(3, 4);
    assert_eq!(base, (3, 4));

    base += (Rows(1), Columns(-1));
    assert_eq!(base, (4, 3));
    base += (Columns(5), Rows(6));
    assert_eq!(base, (10, 8));

    base += (-2, 3);
    assert_eq!(base, (8, 11));

    base += Rows(10);
    assert_eq!(base, (18, 11));
    base += Columns(3);
    assert_eq!(base, (18, 14));

    base += Up;
    assert_eq!(base, (17, 14));
    base += Right;
    assert_eq!(base, (17, 15));
    base += Down;
    assert_eq!(base, (18, 15));
    base += Left;
    assert_eq!(base, (18, 14));
}

impl<T: VectorLike> Sub<T> for Vector {
    type Output = Vector;

    #[inline]
    fn sub(self, rhs: T) -> Vector {
        let rhs = rhs.as_vector();
        Vector {
            rows: self.rows - rhs.rows,
            columns: self.columns - rhs.columns,
        }
    }
}

/// Test that Vectors can be subtracted from themseleves, as well as all the things that
/// can be implicitly converted into Vectors
#[test]
fn test_subtract() {
    use crate::direction::*;

    let base = Vector::new(3, 4);
    assert_eq!(base - Vector::new(2, 3), (1, 1));

    assert_eq!(base - (Rows(1), Columns(2)), (2, 2));
    assert_eq!(base - (Columns(1), Rows(2)), (1, 3));

    assert_eq!(base - (1, 2), (2, 2));

    assert_eq!(base - Rows(5), Vector::new(-2, 4));
    assert_eq!(base - Columns(5), Vector::new(3, -1));

    assert_eq!(base - Up, (4, 4));
    assert_eq!(base - Down, (2, 4));
    assert_eq!(base - Left, (3, 5));
    assert_eq!(base - Right, (3, 3));
}

impl<T: VectorLike> SubAssign<T> for Vector {
    #[inline]
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.as_vector();

        self.rows -= rhs.rows;
        self.columns -= rhs.columns;
    }
}

#[test]
fn test_sub_assign() {
    use crate::direction::*;

    let mut base = Vector::new(24, 26);

    base -= Vector::new(3, 4);
    assert_eq!(base, (21, 22));

    base -= (Rows(1), Columns(-1));
    assert_eq!(base, (20, 23));
    base -= (Columns(5), Rows(6));
    assert_eq!(base, (14, 18));

    base -= (-2, 3);
    assert_eq!(base, (16, 15));

    base -= Rows(10);
    assert_eq!(base, (6, 15));
    base -= Columns(3);
    assert_eq!(base, (6, 12));

    base -= Up;
    assert_eq!(base, (7, 12));
    base -= Right;
    assert_eq!(base, (7, 11));
    base -= Down;
    assert_eq!(base, (6, 11));
    base -= Left;
    assert_eq!(base, (6, 12));
}

/// Multiply a vector's components by a constant factor
impl<T: Copy> Mul<T> for Vector
where
    isize: Mul<T, Output = isize>,
{
    type Output = Vector;

    #[inline]
    fn mul(self, factor: T) -> Vector {
        Vector {
            rows: self.rows * factor,
            columns: self.columns * factor,
        }
    }
}

#[test]
fn test_mul() {
    assert_eq!(Vector::new(2, 3) * 4, (8, 12));
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

#[test]
fn test_mul_assign() {
    let mut base = Vector::new(-2, 3);

    base *= 2;
    base *= -1;

    assert_eq!(base, (4, -6));
}

impl Neg for Vector {
    type Output = Vector;

    #[inline]
    fn neg(self) -> Vector {
        self.reverse()
    }
}

#[test]
fn test_neg() {
    let base = Vector::new(4, -1);
    assert_eq!(-base, (-4, 1));
}

impl<T: VectorLike> PartialEq<T> for Vector {
    fn eq(&self, rhs: &T) -> bool {
        let rhs = rhs.as_vector();
        self.rows == rhs.rows && self.columns == rhs.columns
    }
}

#[test]
fn test_eq() {
    use crate::direction::*;

    let base = Vector::new(2, 3);

    assert!(base == Vector::new(2, 3));

    assert!(base == (Rows(2), Columns(3)));
    assert!(base == (Columns(3), Rows(2)));

    assert!(base == (2, 3));

    assert!(Vector::new(3, 0) == Rows(3));
    assert!(Vector::new(0, 2) == Columns(2));

    assert!(Vector::new(1, 0) == Down);
    assert!(Vector::new(-1, 0) == Up);
    assert!(Vector::new(0, 1) == Right);
    assert!(Vector::new(0, -1) == Left);

    assert!(Vector::new(1, 1) == Down + Right);
}

impl<T: VectorLike> PartialOrd<T> for Vector {
    fn partial_cmp(&self, rhs: &T) -> Option<Ordering> {
        let rhs = rhs.as_vector();
        match (self.rows.cmp(&rhs.rows), self.columns.cmp(&rhs.columns)) {
            (Ordering::Greater, Ordering::Less) | (Ordering::Less, Ordering::Greater) => None,
            (o1, o2) => Some(o1.then(o2)),
        }
    }
}

impl<T: VectorLike> Sum<T> for Vector {
    fn sum<I: Iterator<Item = T>>(iter: I) -> Self {
        iter.fold(Vector::zero(), Add::add)
    }
}

/// This array contains unit vectors associated with the 4 orthogonally
/// adjacent directions. It is intended to allow for easy iteration over
/// orthogonally adjacent locations. The order of the vectors is unspecified
/// and should not be relied upon.
///
/// # Example
///
/// ```
/// use gridly::prelude::*;
/// use gridly::shorthand::*;
/// let root = L(1, 2);
/// let adjacent: Vec<Location> = ORTHOGONAL_ADJACENCIES.iter().map(|v| root + v).collect();
///
/// assert!(adjacent.contains(&L(0, 2)));
/// assert!(adjacent.contains(&L(2, 2)));
/// assert!(adjacent.contains(&L(1, 3)));
/// assert!(adjacent.contains(&L(1, 1)));
/// assert_eq!(adjacent.len(), 4);
/// ```
pub static ORTHOGONAL_ADJACENCIES: [Vector; 4] = [
    Vector::upward(1),
    Vector::rightward(1),
    Vector::downward(1),
    Vector::leftward(1),
];

/// This array contains unit vectors associated with the 4 diagonal directions.
/// It is intended to allow for easy iteration over diagonally adjacent
/// locations. The order of the vectors is unspecified and should not be
/// relied upon.
///
/// # Example
///
/// ```
/// use gridly::prelude::*;
/// use gridly::shorthand::*;
/// let root = L(1, 2);
/// let corners: Vec<Location> = DIAGONAL_ADJACENCIES.iter().map(|v| root + v).collect();
///
/// assert!(corners.contains(&L(0, 1)));
/// assert!(corners.contains(&L(0, 3)));
/// assert!(corners.contains(&L(2, 3)));
/// assert!(corners.contains(&L(2, 1)));
/// assert_eq!(corners.len(), 4);
/// ```
pub static DIAGONAL_ADJACENCIES: [Vector; 4] = [
    Vector::new_const(-1, 1),
    Vector::new_const(1, 1),
    Vector::new_const(1, -1),
    Vector::new_const(-1, -1),
];

/// This array contains unit vectors associated with the 8 adjacent directions.
/// It is intended to allow for easy iteration over all locations that touch
/// a center location (for instance, when scanning adjacencies in an
/// implementation of [Conway's Game of Life]). The order of the vectors
/// is unspecified and should not be relied upon.
///
/// # Example
///
/// ```
/// use gridly::prelude::*;
/// use gridly::shorthand::*;
/// let root = L(1, 2);
/// let touching: Vec<Location> = TOUCHING_ADJACENCIES.iter().map(|v| root + v).collect();
///
/// assert!(touching.contains(&L(0, 1)));
/// assert!(touching.contains(&L(0, 3)));
/// assert!(touching.contains(&L(2, 3)));
/// assert!(touching.contains(&L(2, 1)));
/// assert!(touching.contains(&L(0, 2)));
/// assert!(touching.contains(&L(2, 2)));
/// assert!(touching.contains(&L(1, 3)));
/// assert!(touching.contains(&L(1, 1)));
/// assert_eq!(touching.len(), 8);
/// ```
///
/// <sup>Death to the false Emperor</sup>
///
/// [Conway's Game of Life]: https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life
pub static TOUCHING_ADJACENCIES: [Vector; 8] = [
    Vector::new_const(-1, 0),
    Vector::new_const(-1, 1),
    Vector::new_const(0, 1),
    Vector::new_const(1, 1),
    Vector::new_const(1, 0),
    Vector::new_const(1, -1),
    Vector::new_const(0, -1),
    Vector::new_const(-1, -1),
];

// TODO: in principle all 4 of these arrays could overlap each other. Any
// way to do that without a slice?
