use core::ops::{Mul, Neg};

use crate::vector::Vector;

/// The four cardinal directions (up, down, left, right).
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

pub use self::Direction::{Down, Left, Right, Up};

impl Direction {
    /// Return a vector with the given length in this direction
    pub fn sized_vec(self, length: isize) -> Vector {
        match self {
            Up => Vector::upward(length),
            Down => Vector::downward(length),
            Left => Vector::leftward(length),
            Right => Vector::rightward(length),
        }
    }

    /// Return the unit vector in the given direction
    #[inline]
    pub fn unit_vec(self) -> Vector {
        self.sized_vec(1)
    }

    /// True if this is `Up` or `Down`
    #[inline]
    pub fn is_vertical(self) -> bool {
        match self {
            Up | Down => true,
            Left | Right => false,
        }
    }

    /// True if this is `Left` or `Right`
    #[inline]
    pub fn is_horizontal(self) -> bool {
        !self.is_vertical()
    }

    /// Reverse this direction (`Up` -> `Down`, etc)
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
    #[inline]
    pub fn counter_clockwise(self) -> Direction {
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }
}

impl Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Direction {
        self.reverse()
    }
}

impl Mul<isize> for Direction {
    type Output = Vector;

    fn mul(self, amount: isize) -> Vector {
        self.sized_vec(amount)
    }
}
