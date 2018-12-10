use std::ops::Neg;

use crate::vector::Vector;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub use self::Direction::*;

impl Direction {
    /// Return the unit vector in the given direction
    #[inline]
    pub fn unit_vec(&self) -> Vector {
        match self {
            Up => Vector::upward(1),
            Down => Vector::downward(1),
            Left => Vector::leftward(1),
            Right => Vector::rightward(1),
        }
    }

    /// True if this is Up or Down
    #[inline]
    pub fn is_vertical(&self) -> bool {
        match self {
            Up | Down => true,
            Left | Right => false,
        }
    }

    /// True if this is Left or Right
    #[inline]
    pub fn is_horizontal(&self) -> bool {
        !self.is_vertical()
    }

    /// Reverse this direction (Up -> Down, etc)
    #[inline]
    pub fn reverse(&self) -> Direction {
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }

    /// Rotate this direction clockwise
    #[inline]
    pub fn clockwise(&self) -> Direction {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    /// Rotate this direction counterclockwise
    #[inline]
    pub fn counter_clockwise(&self) -> Direction {
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
