//! A simple enumeration for the 4 cardinal directions.

use core::ops::{Add, Mul, Neg, Sub};

use crate::vector::{Vector, VectorLike, Rows, Columns};

/// The four cardinal directions (up, down, left, right). `Direction`
/// implements a number of simple helper methods. It also implements
/// [`VectorLike`], which allows it to be used in contexts where a [`Vector`]
/// can be used as a unit vector in the given direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    /// The negative row direction
    Up,

    /// The positive row direction
    Down,

    /// The negative column direction
    Left,

    /// The positive column direction
    Right,
}

pub use Direction::{Down, Left, Right, Up};

impl Direction {
    /// Return a vector with the given length in this direction
    ///
    /// # Example:
    ///
    /// ```
    /// use gridly::prelude::*;
    ///
    /// assert_eq!(Up.sized_vec(2), Vector::new(-2, 0));
    /// assert_eq!(Down.sized_vec(3), Vector::new(3, 0));
    /// assert_eq!(Left.sized_vec(1), Vector::new(0, -1));
    /// assert_eq!(Right.sized_vec(5), Vector::new(0, 5));
    /// ```
    pub fn sized_vec(self, length: isize) -> Vector {
        match self {
            Up => Vector::upward(length),
            Down => Vector::downward(length),
            Left => Vector::leftward(length),
            Right => Vector::rightward(length),
        }
    }

    /// Return the unit vector in the given direction.
    ///
    /// # Example:
    ///
    /// ```
    /// use gridly::prelude::*;
    ///
    /// assert_eq!(Up.unit_vec(), Vector::new(-1, 0));
    /// assert_eq!(Down.unit_vec(), Vector::new(1, 0));
    /// assert_eq!(Left.unit_vec(), Vector::new(0, -1));
    /// assert_eq!(Right.unit_vec(), Vector::new(0, 1));
    /// ```
    #[inline]
    pub fn unit_vec(self) -> Vector {
        self.sized_vec(1)
    }

    /// True if this is `Up` or `Down`
    ///
    /// # Example:
    ///
    /// ```
    /// use gridly::direction::*;
    ///
    /// assert!(Up.is_vertical());
    /// assert!(Down.is_vertical());
    /// assert!(!Left.is_vertical());
    /// assert!(!Right.is_vertical());
    /// ```
    #[inline]
    pub fn is_vertical(self) -> bool {
        match self {
            Up | Down => true,
            Left | Right => false,
        }
    }

    /// True if this is `Left` or `Right`
    ///
    /// # Example:
    ///
    /// ```
    /// use gridly::direction::*;
    ///
    /// assert!(!Up.is_horizontal());
    /// assert!(!Down.is_horizontal());
    /// assert!(Left.is_horizontal());
    /// assert!(Right.is_horizontal());
    /// ```
    #[inline]
    pub fn is_horizontal(self) -> bool {
        !self.is_vertical()
    }

    /// Reverse this direction (`Up` -> `Down`, etc)
    ///
    /// ```
    /// use gridly::direction::*;
    ///
    /// assert_eq!(Up.reverse(), Down);
    /// assert_eq!(Down.reverse(), Up);
    /// assert_eq!(Left.reverse(), Right);
    /// assert_eq!(Right.reverse(), Left);
    /// ```
    #[inline]
    pub fn reverse(self) -> Direction {
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }

    /// Rotate this direction clockwise
    ///
    /// # Example:
    ///
    /// ```
    /// use gridly::direction::*;
    ///
    /// assert_eq!(Up.clockwise(), Right);
    /// assert_eq!(Down.clockwise(), Left);
    /// assert_eq!(Left.clockwise(), Up);
    /// assert_eq!(Right.clockwise(), Down);
    /// ```
    #[inline]
    pub fn clockwise(self) -> Direction {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    /// Rotate this direction counterclockwise
    ///
    /// # Example:
    ///
    /// ```
    /// use gridly::direction::*;
    ///
    /// assert_eq!(Up.anticlockwise(), Left);
    /// assert_eq!(Down.anticlockwise(), Right);
    /// assert_eq!(Left.anticlockwise(), Down);
    /// assert_eq!(Right.anticlockwise(), Up);
    /// ```
    #[inline]
    pub fn anticlockwise(self) -> Direction {
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }
}

/// Adding a `Vector` to a `Direction` is equivelent to adding it to a
/// unit vector in the given direction
///
/// # Example:
///
/// ```
/// use gridly::vector::Vector;
/// use gridly::direction::*;
///
/// let base = Vector::new(3, 4);
///
/// assert_eq!(Up + base, Vector::new(2, 4));
/// assert_eq!(Down + base, Vector::new(4, 4));
/// assert_eq!(Right + base, Vector::new(3, 5));
/// assert_eq!(Left + base, Vector::new(3, 3));
/// ```
impl<T: VectorLike> Add<T> for Direction {
    type Output = Vector;

    #[inline]
    fn add(self, rhs: T) -> Vector {
        // TODO: is it more efficient to do a match here?
        rhs.as_vector() + self.unit_vec()
    }
}

/// Subtracting a `Vector` from a `Direction` is equivelent to subtracing
/// it from a unit vector in the given direction
///
/// # Example:
///
/// ```
/// use gridly::vector::Vector;
/// use gridly::direction::*;
///
/// let base = Vector::new(0, 0);
///
/// assert_eq!(Up - base, Vector::new(-1, 0));
/// assert_eq!(Down - base, Vector::new(1, 0));
/// assert_eq!(Right - base, Vector::new(0, 1));
/// assert_eq!(Left - base, Vector::new(0, -1));
/// ```
impl<T: VectorLike> Sub<T> for Direction {
    type Output = Vector;

    #[inline]
    fn sub(self, rhs: T) -> Vector {
        self.unit_vec() - rhs.as_vector()
    }
}

/// Negating a `Direction` reverses it
///
/// # Example:
///
/// ```
/// use gridly::direction::*;
///
/// assert_eq!(-Up, Down);
/// assert_eq!(-Down, Up);
/// assert_eq!(-Left, Right);
/// assert_eq!(-Right, Left);
/// ```
impl Neg for Direction {
    type Output = Direction;

    #[inline]
    fn neg(self) -> Direction {
        self.reverse()
    }
}

/// Multiplying a `Direction` by an `isize` produces a Vector of the given
/// length in the given direction
///
/// # Example:
///
/// ```
/// use gridly::direction::*;
/// use gridly::vector::Vector;
///
/// assert_eq!(Up * 5, Vector::new(-5, 0));
/// assert_eq!(Down * 3, Vector::new(3, 0));
/// assert_eq!(Left * 2, Vector::new(0, -2));
/// assert_eq!(Right * 4, Vector::new(0, 4));
/// ```
impl Mul<isize> for Direction {
    type Output = Vector;

    #[inline]
    fn mul(self, amount: isize) -> Vector {
        self.sized_vec(amount)
    }
}

/// A `Direction` acts like a unit vector in the given direction
impl VectorLike for Direction {
    #[inline]
    fn rows(&self) -> Rows {
        match self {
            Up => Rows(-1),
            Down => Rows(1),
            Left | Right => Rows(0)
        }
    }

    #[inline]
    fn columns(&self) -> Columns {
        match self {
            Left => Columns(-1),
            Right => Columns(1),
            Up | Down => Columns(0)
        }
    }

    #[inline]
    fn as_vector(&self) -> Vector {
        self.unit_vec()
    }
}

#[test]
fn test_vectorlike_direction() {
    // Test that the manual implementations of `rows`, `columns`, and `as_vector`
    // are all compatible.
    for direction in &[Up, Down, Left, Right] {
        assert_eq!(direction.rows() + direction.columns(), direction.as_vector())
    }
}
