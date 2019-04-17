mod range;

use core::fmt::Debug;
use core::hash::Hash;
use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::location::Location;
use crate::vector::{Columns, Component as VecComponent, Rows};

pub use range::{ColumnRange, ColumnRangeError, Range, RangeError, RowRange, RowRangeError};
// TODO: add additional implied traits
// TODO: docstrings

/// A component of a [`Location`]
///
/// This trait comprises a component of a [`Location`], which may be either a
/// [`Row`] or a [`Column`]. It is effectively an index into a given row or
/// column of a grid; for instance, a `Row` can index a column in a grid.
///
/// In practice, most code will call methods directly on [`Row`] or [`Column`]
/// values. However, a lot of gridly and functionality is agnostic towards
/// rows and columns (for instance, a view over a row in a grid is functionally
/// the same as a view over a column), so the `Component` trait allows such
/// functionality to be written generically.
///
/// The key methods for `Component` that allow it to work in generic contexts
/// are `from_location`, which gets a component from a `Location`, and `combine`,
/// which combines a `Row` or `Column` with its converse (a `Column` or a `Row`)
/// to create a new `Location`.
pub trait Component: Sized + From<isize> + Copy + Debug + Ord + Eq + Hash {
    /// The converse component ([`Row`] to [`Column`], or vice versa)
    type Converse: Component<Converse = Self>;

    /// The associated vector component ([`Rows`] or [`Columns`])
    type Distance: VecComponent;

    /// Get this component type from a [`Location`].
    ///
    /// Example:
    ///
    /// ```
    /// use gridly::location::{Component, Row, Column, Location};
    ///
    /// let loc = Location::new(2, 5);
    /// assert_eq!(Row::from_location(&loc), Row(2));
    /// assert_eq!(Column::from_location(&loc), Column(5));
    /// ```
    fn from_location(location: &Location) -> Self;

    /// Combine this component with its converse to create a [`Location`]. Note
    /// that `Row` and `Column` can simply be added together with `+` to create
    /// a new `Location`; this method exists to assist with generic code.
    ///
    /// Example:
    ///
    /// ```
    /// use gridly::location::{Component, Row, Column, Location};
    ///
    /// let loc = Row(2).combine(Column(5));
    /// assert_eq!(loc, (2, 5).into());
    /// assert_eq!(loc, Row(2) + Column(5));
    /// ```
    fn combine(self, other: Self::Converse) -> Location;

    /// Return the lowercase name of this component type– "row" or "column".
    /// Intended for debug printing, error messages, etc.
    fn name() -> &'static str;

    /// Get the integer value of this component
    fn value(self) -> isize;

    /// Add a distance to this component.
    fn add(self, amount: Self::Distance) -> Self;

    /// Find the distance between two components, using this component as the origin
    fn distance_to(self, target: Self) -> Self::Distance {
        target.distance_from(self)
    }

    /// Find the distance between two components, using the other component as the origin
    fn distance_from(self, origin: Self) -> Self::Distance;

    /// Convert a Row into a Column or vice versa
    fn transpose(self) -> Self::Converse {
        self.value().into()
    }
}

// TODO: TryFrom<usize>, once it's stable

// TODO: add docstrings to these. Perhaps refer back to Component
macro_rules! make_component {
    (
        // The component type (Row or Column)
        $Name:ident,

        // The converse type (Column or Row)
        $Converse:ident,

        // The equivelent distance type (Rows or Columns)
        $Distance:ident,

        // The field to use when getting from a location
        $from_loc:ident,

        // Rules for converting into a (row, column) pair.
        ($self:ident, $other:ident) =>
        ($first:ident, $second:ident),

        // The string name of the component
        $name:literal
    ) => {
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $Name(pub isize);

        /// Adding a component to its converse (ie, a [`Row`] to a [`Column`])
        /// produces a [`Location`]
        impl Add<$Converse> for $Name {
            type Output = Location;

            fn add(self, rhs: $Converse) -> Location {
                self.combine(rhs)
            }
        }

        impl Add<$Distance> for $Name {
            type Output = Self;

            fn add(self, rhs: $Distance) -> Self {
                $Name(self.0 + rhs.0)
            }
        }

        impl AddAssign<$Distance> for $Name {
            fn add_assign(&mut self, rhs: $Distance) {
                self.0 += rhs.0
            }
        }

        impl Sub<$Distance> for $Name {
            type Output = Self;

            fn sub(self, rhs: $Distance) -> Self {
                $Name(self.0 - rhs.0)
            }
        }

        impl SubAssign<$Distance> for $Name {
            fn sub_assign(&mut self, rhs: $Distance) {
                self.0 -= rhs.0
            }
        }

        /// The difference between two location components is the distance between them
        impl Sub<$Name> for $Name {
            type Output = $Distance;

            fn sub(self, rhs: Self) -> $Distance {
                $Distance(self.0 - rhs.0)
            }
        }

        impl From<isize> for $Name {
            fn from(value: isize) -> Self {
                $Name(value)
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

            fn add(self, distance: Self::Distance) -> Self {
                self + distance
            }

            fn distance_from(self, origin: Self) -> Self::Distance {
                self - origin
            }

            fn value(self) -> isize {
                self.0
            }
        }
    }
}

make_component! {Row,    Column, Rows,    row,    (self, other) => (self, other), "row"}
make_component! {Column, Row,    Columns, column, (self, other) => (other, self), "column"}
