use core::cmp::Ordering;
use core::fmt::Debug;
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::location::{Column, Component as LocComponent, Row};
use crate::vector::Vector;

//TODO: Add all the remaining traits to Component
//
/// A [`Rows`] or [`Columns`] component of a [`Vector`]
pub trait Component:
    Sized
    + From<isize>
    + Copy
    + Ord
    + Eq
    + Debug
    + Add
    + Sub
    + Neg
    + Default
    + PartialEq<isize>
    + PartialOrd<isize>
{
    /// The converse component ([`Rows`] to [`Columns`] or vice versa)
    type Converse: Component<Converse = Self>;

    /// The assoicated location component type ([`Row`] or [`Column`])
    type Point: LocComponent<Distance = Self>;

    /// Get this compnent from a [`Vector`]
    fn from_vector(vector: &Vector) -> Self;

    /// Create a vector from a Row and Column
    fn combine(self, converse: Self::Converse) -> Vector;

    /// Return the lowercase name of this type of component, "rows" or "columns".
    fn name() -> &'static str;

    /// Get the integer value of this component
    fn value(self) -> isize;

    // Convert a Row into a Column or vice versa
    fn transpose(self) -> Self::Converse;
}

// TODO: add docstrings to these. Perhaps refer back to Component
macro_rules! make_component {
    (
        $Name:ident,
        $Converse:ident,
        $Point:ident,
        $from_vec:ident,
        ($self:ident, $other:ident) =>
        ($first:ident, $second:ident),
        $name: expr
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
        )]
        pub struct $Name(pub isize);

        /// Adding a component to its converse (ie, [`Rows`] to [`Columns`])
        /// creates a Vector
        impl Add<$Converse> for $Name {
            type Output = Vector;

            fn add(self, rhs: $Converse) -> Vector {
                self.combine(rhs)
            }
        }

        impl<T: Into<$Name>> Add<T> for $Name {
            type Output = Self;

            fn add(self, rhs: T) -> Self {
                $Name(self.0 + rhs.into().0)
            }
        }

        impl<T: Into<$Name>> AddAssign<T> for $Name {
            fn add_assign(&mut self, rhs: T) {
                self.0 += rhs.into().0
            }
        }

        impl<T: Into<$Name>> Sub<T> for $Name {
            type Output = Self;

            fn sub(self, rhs: T) -> Self {
                $Name(self.0 - rhs.into().0)
            }
        }

        impl<T: Into<$Name>> SubAssign<T> for $Name {
            fn sub_assign(&mut self, rhs: T) {
                self.0 -= rhs.into().0
            }
        }

        impl<T> Mul<T> for $Name
            where isize: Mul<T, Output=isize>
        {
            type Output = Self;

            fn mul(self, factor: T) -> Self {
                $Name(self.0 * factor)
            }
        }

        impl<T> MulAssign<T> for $Name
            where isize: MulAssign<T>
        {
            fn mul_assign(&mut self, factor: T) {
                self.0 *= factor
            }
        }

        impl Neg for $Name {
            type Output = Self;

            fn neg(self) -> Self {
                $Name(-self.0)
            }
        }

        impl From<isize> for $Name {
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

        impl Component for $Name {
            type Converse = $Converse;
            type Point = $Point;

            fn from_vector(vector: &Vector) -> Self {
                vector.$from_vec
            }

            fn combine($self, $other: $Converse) -> Vector {
                Vector::new($first, $second)
            }

            fn name() -> &'static str {
                $name
            }

            fn value(self) -> isize {
                self.0
            }

            fn transpose(self) -> Self::Converse {
                $Converse(self.0)
            }
        }
    }
}

make_component! {Rows, Columns, Row, rows, (self, other) => (self, other), "rows"}
make_component! {Columns, Rows, Column, columns, (self, other) => (other, self), "columns"}
