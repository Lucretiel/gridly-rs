//! A simple enumeration for the 4 cardinal directions.

use core::ops::{Add, Mul, Neg, Sub};

use crate::vector::{Columns, Rows, Vector, VectorLike};

/// The four cardinal directions (up, down, left, right). [`Direction`]
/// implements a number of simple helper methods. It also implements
/// [`VectorLike`], which allows it to be used in contexts where a [`Vector`]
/// can be used as a unit vector in the given direction (for example, with
/// Vector arithmetic).
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

macro_rules! string_match {
    ($input:expr => $($($pattern:literal)+ => $result:expr;)*) => {
        if false {None}
        $($(
            else if $input.eq_ignore_ascii_case($pattern) {Some($result)}
        )*)*
        else {None}
    }
}

impl Direction {
    /// Parse a direction name into a direction. Currently supported
    /// names are (case insensitive):
    /// - `Up`: Up, North, U, N
    /// - `Down`: Down, South, D, S
    /// - `Left`: Left, West, L, W
    /// - `Right`: Right, East, R, E
    ///
    /// # Example
    ///
    /// ```
    /// use gridly::prelude::*;
    ///
    /// assert_eq!(Direction::from_name("up"), Some(Up));
    /// assert_eq!(Direction::from_name("West"), Some(Left));
    /// assert_eq!(Direction::from_name("Foo"), None);
    /// ```
    #[must_use]
    #[inline]
    pub fn from_name(name: &str) -> Option<Self> {
        string_match! {
            name =>
                "up"    "u" "north" "n" => Up;
                "down"  "d" "south" "s" => Down;
                "left"  "l" "west"  "w" => Left;
                "right" "r" "east"  "e" => Right;
        }
    }

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
    #[must_use]
    #[inline]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
    #[inline]
    pub fn is_horizontal(self) -> bool {
        !self.is_vertical()
    }

    /// Reverse this direction (`Up` → `Down`, etc)
    ///
    /// ```
    /// use gridly::direction::*;
    ///
    /// assert_eq!(Up.reverse(), Down);
    /// assert_eq!(Down.reverse(), Up);
    /// assert_eq!(Left.reverse(), Right);
    /// assert_eq!(Right.reverse(), Left);
    /// ```
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
/// unit vector in the given direction. Note that, because [`Direction`]
/// itself implements `VectorLike`, this means you can add together a sequence
/// of directions to get a Vector.
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
///
/// assert_eq!(Up + Right + Up + Up, Vector::new(-3, 1));
/// ```
impl<T: VectorLike> Add<T> for Direction {
    type Output = Vector;

    #[must_use]
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
/// let base = Vector::new(3, 3);
///
/// assert_eq!(Up - base, Vector::new(-4, -3));
/// assert_eq!(Down - base, Vector::new(-2, -3));
/// assert_eq!(Right - base, Vector::new(-3, -2));
/// assert_eq!(Left - base, Vector::new(-3, -4));
/// ```
impl<T: VectorLike> Sub<T> for Direction {
    type Output = Vector;

    #[must_use]
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

    #[must_use]
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

    #[must_use]
    #[inline]
    fn mul(self, amount: isize) -> Vector {
        self.sized_vec(amount)
    }
}

/// A `Direction` acts like a unit vector in the given direction. This allows
/// it to be used in things like Vector arithmetic.
///
/// # Example:
///
/// ```
/// use gridly::prelude::*;
///
/// assert_eq!(Vector::new(1, 1) + Up, Vector::new(0, 1));
/// assert_eq!(Location::new(3, 4) - Left, Location::new(3, 5));
/// ```
// TODO: I'm concerned about the performance implications of this impl,
// since idiomatic use of VectorLike allows you to separately call .rows() and
// .columns(). Hopefully the optimizer can notice that and optimize to a single
// check. For now we hope that inlining will allow the compiler to elide
// unnecessary checks, and prefer to use as_vector for internal methods, where
// relevant.
impl VectorLike for Direction {
    #[must_use]
    #[inline]
    fn rows(&self) -> Rows {
        match self {
            Up => Rows(-1),
            Down => Rows(1),
            Left | Right => Rows(0),
        }
    }

    #[must_use]
    #[inline]
    fn columns(&self) -> Columns {
        match self {
            Left => Columns(-1),
            Right => Columns(1),
            Up | Down => Columns(0),
        }
    }

    #[must_use]
    #[inline]
    fn as_vector(&self) -> Vector {
        self.unit_vec()
    }

    #[must_use]
    #[inline(always)]
    fn manhattan_length(&self) -> isize {
        1
    }

    #[must_use]
    #[inline(always)]
    fn checked_manhattan_length(&self) -> Option<isize> {
        Some(1)
    }

    #[must_use]
    #[inline]
    fn clockwise(&self) -> Vector {
        Direction::clockwise(*self).unit_vec()
    }

    #[must_use]
    #[inline]
    fn anticlockwise(&self) -> Vector {
        Direction::anticlockwise(*self).unit_vec()
    }

    #[must_use]
    #[inline]
    fn reverse(&self) -> Vector {
        Direction::reverse(*self).unit_vec()
    }

    #[must_use]
    #[inline]
    fn transpose(&self) -> Vector {
        match self {
            Down => Right,
            Right => Down,
            Up => Left,
            Left => Up,
        }
        .unit_vec()
    }

    #[must_use]
    #[inline]
    fn direction(&self) -> Option<Direction> {
        Some(*self)
    }
}

#[test]
fn test_from_str() {
    for variant in &[
        "up", "u", "north", "n", "UP", "U", "NORTH", "N", "Up", "U", "North", "N",
    ] {
        assert_eq!(Direction::from_name(variant), Some(Up));
    }

    for variant in &[
        "down", "d", "south", "s", "DOWN", "D", "SOUTH", "S", "Down", "D", "South", "S",
    ] {
        assert_eq!(Direction::from_name(variant), Some(Down));
    }

    for variant in &[
        "left", "l", "west", "w", "LEFT", "L", "WEST", "W", "Left", "L", "West", "W",
    ] {
        assert_eq!(Direction::from_name(variant), Some(Left));
    }

    for variant in &[
        "right", "r", "east", "e", "RIGHT", "R", "EAST", "E", "Right", "R", "East", "E",
    ] {
        assert_eq!(Direction::from_name(variant), Some(Right));
    }

    assert_eq!(Direction::from_name("foo"), None);
}

#[cfg(test)]
mod test_vectorlike {
    use crate::direction::EACH_DIRECTION;
    use crate::vector::VectorLike;

    /// Test that the manual implementations of `rows`, `columns`, and
    /// `as_vector` are all compatible.
    #[test]
    fn test_row_column_vector_compatible() {
        for direction in &EACH_DIRECTION {
            assert_eq!(
                direction.rows() + direction.columns(),
                direction.as_vector()
            );
            assert_eq!(direction.as_vector(), direction.unit_vec());
        }
    }

    mod custom_impls {
        use crate::direction::EACH_DIRECTION;
        use crate::vector::VectorLike;

        /// Direction has a custom implementation of all the VectorLike
        /// methods. These tests confirm that the custom implementations match
        /// the vector versions.
        macro_rules! test_vectorlike_method {
            ($method:ident) => {
                #[test]
                fn $method() {
                    for direction in &EACH_DIRECTION {
                        let vector = direction.unit_vec();

                        assert_eq!(vector.$method(), VectorLike::$method(direction),);
                    }
                }
            };
        }

        test_vectorlike_method! {manhattan_length}
        test_vectorlike_method! {checked_manhattan_length}
        test_vectorlike_method! {clockwise}
        test_vectorlike_method! {anticlockwise}
        test_vectorlike_method! {reverse}
        test_vectorlike_method! {transpose}
        test_vectorlike_method! {direction}
    }
}

/// This array contains each direction; it is intended to allow for easy
/// iteration over adjacent locations. The order of the directions is
/// left unspecified and should not be relied upon.
///
/// # Example
///
/// ```
/// use gridly::prelude::*;
/// use gridly::shorthand::*;
/// let root = L(1, 2);
/// let adjacent: Vec<Location> = EACH_DIRECTION.iter().map(|v| root + v).collect();
///
/// assert!(adjacent.contains(&L(0, 2)));
/// assert!(adjacent.contains(&L(2, 2)));
/// assert!(adjacent.contains(&L(1, 3)));
/// assert!(adjacent.contains(&L(1, 1)));
/// assert_eq!(adjacent.len(), 4);
/// ```
pub static EACH_DIRECTION: [Direction; 4] = [Up, Right, Down, Left];
