//! A simple enumeration for 90 degree rotations

use core::iter;
use core::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

// Implementation note: the order here is intended to be the same as the
// order of `Direction`, to assist the compiler when combining them.
/// The 4 cardinal rotations: [`None`] (no rotation), [`Flip`],
/// [`Clockwise`], and [`Anticlockwise`]. These rotations support basic
/// arithmetic operations with themselves (sums, multiplication by a factor,
/// etc) and can be applied to [`Direction`][crate::direction::Direction]
/// and [`Vector`][crate::vector::Vector].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rotation {
    /// No rotation (0°)
    None = 0,

    /// Rotate 90° clockwise
    Clockwise = 1,

    /// Rotate 180°
    Flip = 2,

    /// Rotate 90° anticlockwise
    Anticlockwise = 3,
}

pub use Rotation::{Anticlockwise, Clockwise};
use Rotation::{Flip, None};

impl Rotation {
    /// This function helps us compose [`Rotation`] sums, because a rotation can
    /// always be represented as a sum of int values % 4
    #[inline(always)]
    #[must_use]
    fn as_int(self) -> i8 {
        self as i8
    }

    /// Counterpart to as_int. 0 is [`Rotation::None`], then counts clockwise. Panics if
    /// the input isn't in 0..4, but we assume that with inlining this will
    /// usually be optimized away.
    #[inline(always)]
    #[must_use]
    fn from_int(value: i8) -> Self {
        match value {
            0 => None,
            1 => Clockwise,
            2 => Flip,
            3 => Anticlockwise,
            value => panic!("Invalid numerical direction value: {}", value),
        }
    }

    /// Check if a rotation is a turn; that is, if it is [`Clockwise`] or
    /// [`Anticlockwise`]
    ///
    /// # Example
    ///
    /// ```
    /// use gridly::rotation::*;
    ///
    /// assert!(!Rotation::None.is_turn());
    /// assert!(!Rotation::Flip.is_turn());
    /// assert!(Clockwise.is_turn());
    /// assert!(Anticlockwise.is_turn());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_turn(self) -> bool {
        match self {
            Clockwise | Anticlockwise => true,
            None | Flip => false,
        }
    }

    /// Get the rotation in the opposite direction. Note that [`Rotation::None`] and
    /// [`Rotation::Flip`] are their own opposites.
    ///
    /// # Example
    ///
    /// ```
    /// use gridly::rotation::*;
    ///
    /// assert_eq!(Rotation::None.reverse(), Rotation::None);
    /// assert_eq!(Rotation::Flip.reverse(), Rotation::Flip);
    /// assert_eq!(Clockwise.reverse(), Anticlockwise);
    /// assert_eq!(Anticlockwise.reverse(), Clockwise);
    /// ```
    #[inline]
    #[must_use]
    pub fn reverse(self) -> Self {
        match self {
            Clockwise => Anticlockwise,
            Anticlockwise => Clockwise,
            straight => straight,
        }
    }
}

impl Add for Rotation {
    type Output = Rotation;

    #[inline]
    #[must_use]
    fn add(self, rhs: Rotation) -> Rotation {
        Self::from_int((self.as_int() + rhs.as_int()).rem_euclid(4))
    }
}

#[test]
fn test_add() {
    assert_eq!(
        Clockwise + Clockwise + Anticlockwise + Flip + None + Clockwise + Anticlockwise,
        Anticlockwise
    );
}

impl AddAssign for Rotation {
    #[inline]
    fn add_assign(&mut self, rhs: Rotation) {
        *self = *self + rhs;
    }
}

#[test]
fn test_add_assign() {
    let mut rot = Clockwise;
    rot += Flip;
    rot += None;
    assert_eq!(rot, Anticlockwise);
}

impl Sub for Rotation {
    type Output = Rotation;

    #[inline]
    #[must_use]
    fn sub(self, rhs: Rotation) -> Rotation {
        Self::from_int((self.as_int() - rhs.as_int()).rem_euclid(4))
    }
}

#[test]
fn test_sub() {
    assert_eq!(Anticlockwise - Clockwise, Flip);
    assert_eq!(Flip - None, Flip);
    assert_eq!(Clockwise - Flip, Anticlockwise);
}

impl SubAssign for Rotation {
    #[inline]
    fn sub_assign(&mut self, rhs: Rotation) {
        *self = *self - rhs;
    }
}

#[test]
fn test_sub_assign() {
    let mut rot = Anticlockwise;

    rot -= Flip;
    rot -= Anticlockwise;

    assert_eq!(rot, Flip);
}

/// Macro is the easiest way to overload this for integers
macro_rules! is_even {
    ($value:expr) => {
        $value & 1 == 0
    };
}

macro_rules! impl_mul {
    ($($int:ident)*) => {$(
        impl Mul<$int> for Rotation {
            type Output = Rotation;

            #[inline]
            #[must_use]
            fn mul(self, rhs: $int) -> Rotation {
                match self {
                    None => None,
                    Flip if is_even!(rhs) => None,
                    Flip => Flip,
                    Clockwise => Self::from_int(rhs.rem_euclid(4) as i8),
                    Anticlockwise => Self::from_int((-(rhs.rem_euclid(4) as i8)).rem_euclid(4)),
                }
            }
        }
    )*}
}

impl_mul! {
    u8 u16 u32 u64 usize
    i8 i16 i32 i64 isize
}

#[cfg(test)]
mod test_mul {
    use crate::rotation::Rotation;
    use Rotation::*;

    #[test]
    fn none() {
        assert_eq!(None * 0, None);
        assert_eq!(None * 1, None);
        assert_eq!(None * 2, None);
        assert_eq!(None * 3, None);

        assert_eq!(None * 4, None);
        assert_eq!(None * 5, None);
        assert_eq!(None * 6, None);
        assert_eq!(None * 7, None);

        assert_eq!(None * -1, None);
        assert_eq!(None * -2, None);
        assert_eq!(None * -3, None);
        assert_eq!(None * -4, None);

        assert_eq!(None * -5, None);
        assert_eq!(None * -6, None);
        assert_eq!(None * -7, None);
        assert_eq!(None * -8, None);
    }

    #[test]
    fn flip() {
        assert_eq!(Flip * 0, None);
        assert_eq!(Flip * 1, Flip);
        assert_eq!(Flip * 2, None);
        assert_eq!(Flip * 3, Flip);

        assert_eq!(Flip * 4, None);
        assert_eq!(Flip * 5, Flip);
        assert_eq!(Flip * 6, None);
        assert_eq!(Flip * 7, Flip);

        assert_eq!(Flip * -1, Flip);
        assert_eq!(Flip * -2, None);
        assert_eq!(Flip * -3, Flip);
        assert_eq!(Flip * -4, None);

        assert_eq!(Flip * -5, Flip);
        assert_eq!(Flip * -6, None);
        assert_eq!(Flip * -7, Flip);
        assert_eq!(Flip * -8, None);
    }

    #[test]
    fn clockwise() {
        assert_eq!(Clockwise * 0, None);
        assert_eq!(Clockwise * 1, Clockwise);
        assert_eq!(Clockwise * 2, Flip);
        assert_eq!(Clockwise * 3, Anticlockwise);

        assert_eq!(Clockwise * 4, None);
        assert_eq!(Clockwise * 5, Clockwise);
        assert_eq!(Clockwise * 6, Flip);
        assert_eq!(Clockwise * 7, Anticlockwise);

        assert_eq!(Clockwise * -1, Anticlockwise);
        assert_eq!(Clockwise * -2, Flip);
        assert_eq!(Clockwise * -3, Clockwise);
        assert_eq!(Clockwise * -4, None);

        assert_eq!(Clockwise * -5, Anticlockwise);
        assert_eq!(Clockwise * -6, Flip);
        assert_eq!(Clockwise * -7, Clockwise);
        assert_eq!(Clockwise * -8, None);
    }

    #[test]
    fn anticlockwise() {
        assert_eq!(Anticlockwise * 0, None);
        assert_eq!(Anticlockwise * 1, Anticlockwise);
        assert_eq!(Anticlockwise * 2, Flip);
        assert_eq!(Anticlockwise * 3, Clockwise);

        assert_eq!(Anticlockwise * 4, None);
        assert_eq!(Anticlockwise * 5, Anticlockwise);
        assert_eq!(Anticlockwise * 6, Flip);
        assert_eq!(Anticlockwise * 7, Clockwise);

        assert_eq!(Anticlockwise * -1, Clockwise);
        assert_eq!(Anticlockwise * -2, Flip);
        assert_eq!(Anticlockwise * -3, Anticlockwise);
        assert_eq!(Anticlockwise * -4, None);

        assert_eq!(Anticlockwise * -5, Clockwise);
        assert_eq!(Anticlockwise * -6, Flip);
        assert_eq!(Anticlockwise * -7, Anticlockwise);
        assert_eq!(Anticlockwise * -8, None);
    }
}

impl iter::Sum<Rotation> for Rotation {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self::from_int(iter.fold(0, |acc, rot| (acc + rot.as_int()).rem_euclid(4)))
    }
}

#[test]
fn test_sum() {
    // sum should be clockwise
    let clocks = iter::repeat(Clockwise).take(21);

    // sum should be clockwise
    let anticlocks = iter::repeat(Anticlockwise).take(43);

    // sum should be none
    let nones = iter::repeat(None).take(1000);

    // sum should be none
    let flips = iter::repeat(Flip).take(2000);

    let all = clocks.chain(anticlocks).chain(nones).chain(flips);
    let sum: Rotation = all.sum();

    assert_eq!(sum, Flip);
}

impl<'a> iter::Sum<&'a Rotation> for Rotation {
    #[inline]
    fn sum<I: Iterator<Item = &'a Rotation>>(iter: I) -> Rotation {
        iter.copied().sum()
    }
}

impl Neg for Rotation {
    type Output = Rotation;

    #[inline]
    #[must_use]
    fn neg(self) -> Self {
        self.reverse()
    }
}

#[test]
fn test_neg() {
    assert_eq!(-Flip, Flip);
    assert_eq!(-None, None);
    assert_eq!(-Clockwise, Anticlockwise);
    assert_eq!(-Anticlockwise, Clockwise);
}
