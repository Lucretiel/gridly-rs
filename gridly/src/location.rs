//! [`Location`][crate::location::Location] type used to index into grids,
//! as well as associated types and traits.

use core::cmp::{Ordering, PartialOrd};
use core::fmt::Debug;
use core::hash::Hash;
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Deref, DerefMut, Sub, SubAssign};

use crate::direction::Direction;
use crate::range::{ComponentRange, LocationRange};
use crate::vector::{Columns, Component as VecComponent, Rows, Vector, VectorLike};

// TODO: add additional implied traits?
// TODO: docstrings

/// A component of a [`Location`], which may be either a [`Row`] or a
/// [`Column`]. It is effectively an index into a given row or column of a
/// grid; for instance, a [`Row`] can index a row in a grid.
///
/// In practice, most code will call methods directly on [`Row`] or [`Column`]
/// values. However, a lot of gridly functionality is agnostic towards
/// rows and columns (for instance, a view over a row in a grid is functionally
/// the same as a view over a column), so the `Component` trait allows such
/// functionality to be written generically.
///
/// The key methods for [`Component`] that allow it to work in generic contexts
/// are [`from_location`][Component::from_location], which gets a component
/// from a [`Location`], and [`combine`][Component::combine], which combines a
/// [`Row`] or [`Column`] with its converse (a [`Column`] or a [`Row`]) to
/// create a new [`Location`].
pub trait Component: Sized + From<isize> + Copy + Debug + Ord + Eq + Hash + Default {
    /// The converse component ([`Row`] to [`Column`], or vice versa)
    type Converse: Component<Converse = Self>;

    /// The associated vector component ([`Rows`] or [`Columns`])
    type Distance: VecComponent<Point = Self>;

    /// Get this component type from a [`Location`].
    ///
    /// Example:
    ///
    /// ```
    /// use gridly::prelude::*;
    ///
    /// let loc = Location::new(2, 5);
    /// assert_eq!(Row::from_location(&loc), Row(2));
    /// assert_eq!(Column::from_location(&loc), Column(5));
    /// ```
    #[must_use]
    fn from_location<L: LocationLike>(location: L) -> Self;

    /// Combine this component with its converse to create a [`Location`]. Note
    /// that `Row` and `Column` can simply be added together with `+` to create
    /// a new `Location`; this method exists to assist with generic code.
    ///
    /// Example:
    ///
    /// ```
    /// use gridly::prelude::*;
    ///
    /// let loc = Row(2).combine(Column(5));
    /// assert_eq!(loc, Location::new(2, 5));
    /// assert_eq!(loc, Row(2) + Column(5));
    /// ```
    #[must_use]
    fn combine(self, other: Self::Converse) -> Location;

    /// Return the lowercase name of this component type– "row" or "column".
    /// Intended for debug printing, error messages, etc.
    ///
    /// ```
    /// use gridly::prelude::*;
    ///
    /// assert_eq!(Row::name(), "row");
    /// assert_eq!(Column::name(), "column");
    /// ```
    #[must_use]
    fn name() -> &'static str;

    /// Get the integer value of this component
    ///
    /// ```
    /// use gridly::prelude::*;
    ///
    /// assert_eq!(Row(5).value(), 5);
    /// ```
    #[must_use]
    fn value(self) -> isize;

    /// Add a distance to this component. This method is provided because we can't
    /// require a trait bound on `Add` for `Component`, but in general just using
    /// `+` is preferable.
    ///
    /// ```
    /// use gridly::prelude::*;
    ///
    /// assert_eq!(Row(4).add_distance(Rows(5)), Row(9));
    /// ```
    #[must_use]
    fn add_distance(self, amount: impl Into<Self::Distance>) -> Self;

    /// Find the distance between two components, using this component as the origin
    ///
    /// ```
    /// use gridly::prelude::*;
    ///
    /// assert_eq!(Row(3).distance_to(Row(8)), Rows(5));
    /// ```
    #[must_use]
    #[inline(always)]
    fn distance_to(self, target: Self) -> Self::Distance {
        target.distance_from(self)
    }

    /// Find the distance between two components, using the other component as the origin
    ///
    /// ```
    /// use gridly::prelude::*;
    ///
    /// assert_eq!(Row(8).distance_from(Row(3)), Rows(5));
    /// ```
    #[must_use]
    fn distance_from(self, origin: Self) -> Self::Distance;

    /// Convert a Row into a Column or vice versa
    ///
    /// ```
    /// use gridly::prelude::*;
    ///
    /// assert_eq!(Row(3).transpose(), Column(3));
    /// ```
    #[must_use]
    #[inline]
    fn transpose(self) -> Self::Converse {
        self.value().into()
    }

    /// Create a range, starting at this component, with the given length
    ///
    /// ```
    /// use gridly::prelude::*;
    ///
    /// assert_eq!(Row(-1).span(Rows(2)), RowRange::bounded(Row(-1), Row(1)));
    /// ```
    #[must_use]
    #[inline]
    fn span(self, length: Self::Distance) -> ComponentRange<Self> {
        ComponentRange::span(self, length)
    }

    /// Create a range, starting at this component, ending at the given
    /// component
    ///
    /// ```
    /// use gridly::prelude::*;
    ///
    /// assert_eq!(Row(10).range_to(Row(20)), RowRange::span(Row(10), Rows(10)));
    /// ```
    #[must_use]
    #[inline]
    fn range_to(self, end: Self) -> ComponentRange<Self> {
        ComponentRange::bounded(self, end)
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
        $lower_name:ident,

        // The converse field when getting from a location
        $lower_converse:ident,

        // The string name of the field, to use in docstrings
        $name:literal,

        // The name of the module in which to place unit tests
        $test:ident
    ) => {
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        #[doc = "A "]
        #[doc = $name]
        #[doc = " component of a [`Location`]. See [`Component`] for details."]
        pub struct $Name(pub isize);

        /// Adding a component to its converse (ie, a [`Row`] to a [`Column`])
        /// produces a [`Location`]
        impl Add<$Converse> for $Name {
            type Output = Location;

            #[must_use]
            #[inline]
            fn add(self, rhs: $Converse) -> Location {
                self.combine(rhs)
            }
        }

        impl Add<$Distance> for $Name {
            type Output = Self;

            #[must_use]
            #[inline]
            fn add(self, rhs: $Distance) -> Self {
                $Name(self.0 + rhs.0)
            }
        }

        impl AddAssign<$Distance> for $Name {
            #[inline]
            fn add_assign(&mut self, rhs: $Distance) {
                self.0 += rhs.0
            }
        }

        impl Sub<$Distance> for $Name {
            type Output = Self;

            #[must_use]
            #[inline]
            fn sub(self, rhs: $Distance) -> Self {
                $Name(self.0 - rhs.0)
            }
        }

        impl SubAssign<$Distance> for $Name {
            #[inline]
            fn sub_assign(&mut self, rhs: $Distance) {
                self.0 -= rhs.0
            }
        }

        /// The difference between two location components is the distance between them
        impl Sub<$Name> for $Name {
            type Output = $Distance;

            #[must_use]
            #[inline]
            fn sub(self, rhs: Self) -> $Distance {
                $Distance(self.0 - rhs.0)
            }
        }

        impl From<isize> for $Name {
            #[must_use]
            #[inline]
            fn from(value: isize) -> Self {
                $Name(value)
            }
        }

        impl LocationLike for ($Name, $Converse) {
            #[inline]
            #[must_use]
            fn $lower_name(&self) -> $Name {
                self.0
            }

            #[inline]
            #[must_use]
            fn $lower_converse(&self) -> $Converse {
                self.1
            }

            #[inline]
            #[must_use]
            fn as_location(&self) -> Location {
                self.0.combine(self.1)
            }
        }

        impl Component for $Name {
            type Converse = $Converse;
            type Distance = $Distance;

            #[inline]
            fn from_location<L: LocationLike>(location: L) -> Self {
                location.$lower_name()
            }

            #[inline]
            fn combine(self, other: $Converse) -> Location {
                Location {
                    $lower_name: self,
                    $lower_converse: other,
                }
            }

            #[inline(always)]
            fn name() -> &'static str {
                stringify!($lower_name)
            }

            #[inline]
            fn add_distance(self, distance: impl Into<$Distance>) -> Self {
                self + distance.into()
            }

            #[inline]
            fn distance_from(self, origin: Self) -> $Distance {
                self - origin
            }

            #[inline]
            fn value(self) -> isize {
                self.0
            }
        }

        #[cfg(test)]
        mod $test {
            use crate::location::{$Converse, $Name, Component, Location};
            use crate::vector::$Distance;

            #[test]
            fn test_combine_converse() {
                let base = $Name(3);
                let converse = $Converse(4);

                assert_eq!(
                    base.combine(converse),
                    Location {
                        $lower_name: base,
                        $lower_converse: converse,
                    }
                );
            }

            #[test]
            fn test_add_converse() {
                let base = $Name(3);
                let converse = $Converse(4);

                assert_eq!(base + converse, base.combine(converse));
            }

            #[test]
            fn test_add_distance() {
                let base = $Name(3);
                let distance = $Distance(4);

                assert_eq!(base + distance, $Name(7));
            }

            #[test]
            fn test_add_assign() {
                let mut base = $Name(3);
                base += $Distance(4);

                assert_eq!(base, $Name(7));
            }

            #[test]
            fn test_sub_distance() {
                let base = $Name(3);
                let distance = $Distance(4);

                assert_eq!(base - distance, $Name(-1));
            }

            #[test]
            fn test_sub_assign() {
                let mut base = $Name(3);
                base -= $Distance(4);

                assert_eq!(base, $Name(-1));
            }

            #[test]
            fn test_sub_self() {
                let origin = $Name(2);
                let remote = $Name(5);

                assert_eq!(remote - origin, $Distance(3));
            }
        }
    };
}

make_component! {Row, Column, Rows, row, column, "row", test_row}
make_component! {Column, Row, Columns, column, row, "column", test_column}

/// A location on a grid. A location is the primary indexing type for a
/// Grid, and represents a single cell on that grid. It is comprised of a
/// [`Row`] and a [`Column`]. Increasing row count downward and increasing
/// columns count rightward.
///
/// Locations support arithmetic operations with [`Vector`]s. They can also be
/// subtracted from each other to produce [`Vector`]s measuring the distance
/// between them.
#[derive(Debug, Clone, Copy, Default, Hash, Eq)]
pub struct Location {
    pub row: Row,
    pub column: Column,
}

impl Location {
    /// Create a new location out of a `row` and a `column`
    #[inline]
    #[must_use]
    pub fn new(row: impl Into<Row>, column: impl Into<Column>) -> Self {
        Location {
            row: row.into(),
            column: column.into(),
        }
    }

    /// Create a new location at `(0, 0)`.
    #[must_use]
    #[inline]
    pub const fn zero() -> Self {
        Location {
            row: Row(0),
            column: Column(0),
        }
    }
}

/// This trait covers structs that act like a [`Location`], such as tuples.
/// See the [`Location`] documentation for more details.
pub trait LocationLike: Sized {
    /// Get the row of this location.
    fn row(&self) -> Row;

    /// Get the column of this location.
    fn column(&self) -> Column;

    /// Convert this object into a [`Location`] struct.
    #[inline]
    #[must_use]
    fn as_location(&self) -> Location {
        Location {
            row: self.row(),
            column: self.column(),
        }
    }

    /// Get either the row or column of a location. This method is useful in
    /// code that is generic over the Row or Column.
    #[inline]
    #[must_use]
    fn get_component<T: Component>(&self) -> T {
        T::from_location(self)
    }

    /// Return the location that is `distance` rows above this one
    ///
    /// ```
    /// use gridly::prelude::*;
    /// use gridly::shorthand::*;
    ///
    /// assert_eq!(L(3, 4).above(2), L(1, 4));
    /// ```
    #[inline]
    #[must_use]
    fn above(&self, distance: impl Into<Rows>) -> Location {
        Location {
            row: self.row() - distance.into(),
            column: self.column(),
        }
    }

    /// Return the location that is `distance` rows below this one
    ///
    /// ```
    /// use gridly::prelude::*;
    /// use gridly::shorthand::*;
    ///
    /// assert_eq!(L(3, 4).below(2), L(5, 4));
    /// ```
    #[inline]
    #[must_use]
    fn below(&self, distance: impl Into<Rows>) -> Location {
        Location {
            row: self.row() + distance.into(),
            column: self.column(),
        }
    }

    /// Return the location that is `distance` columns to the left of this one
    ///
    /// ```
    /// use gridly::prelude::*;
    /// use gridly::shorthand::*;
    ///
    /// assert_eq!(L(3, 4).left(2), L(3, 2));
    /// ```
    #[inline]
    #[must_use]
    fn left(&self, distance: impl Into<Columns>) -> Location {
        Location {
            row: self.row(),
            column: self.column() - distance.into(),
        }
    }

    /// Return the location that is `distance` columns to the right of this one
    ///
    /// ```
    /// use gridly::prelude::*;
    /// use gridly::shorthand::*;
    ///
    /// assert_eq!(L(3, 4).right(2), L(3, 6));
    /// ```
    #[inline]
    #[must_use]
    fn right(&self, distance: impl Into<Columns>) -> Location {
        Location {
            row: self.row(),
            column: self.column() + distance.into(),
        }
    }

    /// Return the location that is `distance` away from this one.
    #[inline]
    #[must_use]
    fn add(&self, distance: impl VectorLike) -> Location {
        self.as_location() + distance
    }

    /// Return the location that is `distance` away in the given `direction`
    ///
    /// ```
    /// use gridly::prelude::*;
    /// use gridly::shorthand::*;
    ///
    /// assert_eq!(Location::zero().relative(Up, 4), L(-4, 0));
    /// assert_eq!(Location::zero().relative(Down, 5), L(5, 0));
    /// assert_eq!(Location::zero().relative(Left, 3), L(0, -3));
    /// assert_eq!(Location::zero().relative(Right, 1), L(0, 1));
    /// ```
    #[inline]
    #[must_use]
    fn relative(&self, direction: Direction, distance: isize) -> Location {
        self.add(direction.sized_vec(distance))
    }

    /// Return the location that is 1 away in the given `direction`
    ///
    /// ```
    /// use gridly::prelude::*;
    /// use gridly::shorthand::*;
    ///
    /// let base = Location::new(2, 4);
    /// assert_eq!(base.step(Up), L(1, 4));
    /// assert_eq!(base.step(Down), L(3, 4));
    /// assert_eq!(base.step(Left), L(2, 3));
    /// assert_eq!(base.step(Right), L(2, 5));
    /// ```
    #[inline]
    #[must_use]
    fn step(&self, direction: Direction) -> Location {
        self.add(direction.unit_vec())
    }

    /// Swap the row and colimn of this Location
    ///
    /// Example:
    ///
    /// ```
    /// use gridly::prelude::*;
    /// use gridly::shorthand::*;
    ///
    /// assert_eq!(L(5, 8).transpose(), L(8, 5));
    /// ```
    #[inline]
    #[must_use]
    fn transpose(&self) -> Location {
        Location {
            row: self.column().transpose(),
            column: self.row().transpose(),
        }
    }

    /// Generically get strictly ordered version of this `Location`. The `Major`
    /// is the ordering; for example, `order_by::<Row>` will create a row-ordered
    /// [`Location`]. See [`row_ordered`][LocationLike::row_ordered] or
    /// [`column_ordered`][LocationLike::column_ordered] for an example.
    #[inline]
    #[must_use]
    fn order_by<Major: Component>(self) -> Ordered<Self, Major> {
        self.into()
    }

    /// Get a strictly row-ordered version of this `Location`; that is, a
    /// location which is ordered by comparing the `row`, then the `column`.
    ///
    /// Example:
    ///
    /// ```
    /// use gridly::prelude::*;
    /// use gridly::shorthand::*;
    ///
    /// let l0 = L(0, 0).row_ordered();
    /// let l1 = L(0, 1).row_ordered();
    /// let l2 = L(1, 0).row_ordered();
    /// let l3 = L(1, 1).row_ordered();
    ///
    /// assert!(l0 < l1 && l0 < l2 && l0 < l3);
    /// assert!(l1 < l2 && l1 < l3);
    /// assert!(l2 < l3);
    /// ```
    #[inline]
    #[must_use]
    fn row_ordered(self) -> RowOrdered<Self> {
        self.order_by()
    }

    /// Get a strictly row-ordered version of this `Location`; that is, a
    /// location which is ordered by comparing the `row`, then the `column`.
    ///
    /// Example:
    ///
    /// ```
    ///
    /// use gridly::prelude::*;
    /// use gridly::shorthand::*;
    ///
    /// let l0 = L(0, 0).column_ordered();
    /// let l1 = L(1, 0).column_ordered();
    /// let l2 = L(0, 1).column_ordered();
    /// let l3 = L(1, 1).column_ordered();
    ///
    /// assert!(l0 < l1 && l0 < l2 && l0 < l3);
    /// assert!(l1 < l2 && l1 < l3);
    /// assert!(l2 < l3);
    ///
    /// ```
    #[inline]
    #[must_use]
    fn column_ordered(self) -> ColumnOrdered<Self> {
        self.order_by()
    }

    /// Create a range, starting at this location, with the given length
    ///
    /// ```
    /// use gridly::prelude::*;
    /// use gridly::shorthand::*;
    ///
    /// let location = L(1, 2);
    /// let range = location.span_over(Rows(3));
    ///
    /// assert_eq!(range, LocationRange::bounded(Column(2), Row(1), Row(4)));
    /// ```
    #[inline]
    #[must_use]
    fn span_over<C: VecComponent>(
        self,
        distance: C,
    ) -> LocationRange<<C::Point as Component>::Converse> {
        LocationRange::rooted(self.as_location(), distance)
    }

    /// Create a range, starting at this component, ending at the given
    /// component
    ///
    /// ```
    /// use gridly::prelude::*;
    /// use gridly::shorthand::*;
    ///
    /// let location = L(1, 2);
    /// let range = location.range_to(Column(6));
    ///
    /// assert_eq!(range, LocationRange::bounded(Row(1), Column(2), Column(6)));
    /// ```
    #[inline]
    #[must_use]
    fn range_to<C: Component>(self, end: C) -> LocationRange<C::Converse> {
        LocationRange::bounded(self.get_component(), self.get_component(), end)
    }
}

impl LocationLike for Location {
    #[inline(always)]
    #[must_use]
    fn row(&self) -> Row {
        self.row
    }

    #[inline(always)]
    #[must_use]
    fn column(&self) -> Column {
        self.column
    }

    #[inline(always)]
    #[must_use]
    fn as_location(&self) -> Location {
        *self
    }
}

impl<T: LocationLike> LocationLike for &T {
    #[inline(always)]
    #[must_use]
    fn row(&self) -> Row {
        T::row(self)
    }

    #[inline(always)]
    #[must_use]
    fn column(&self) -> Column {
        T::column(self)
    }

    #[inline(always)]
    #[must_use]
    fn as_location(&self) -> Location {
        T::as_location(self)
    }
}

impl<T: VectorLike> Add<T> for Location {
    type Output = Location;

    #[inline]
    #[must_use]
    fn add(self, rhs: T) -> Location {
        let rhs = rhs.as_vector();
        Location {
            row: self.row + rhs.rows,
            column: self.column + rhs.columns,
        }
    }
}

#[cfg(test)]
#[test]
fn test_add() {
    use crate::direction::*;

    assert_eq!(
        Location::new(3, 5) + Vector::new(-1, 6),
        Location::new(2, 11)
    );
    assert_eq!(Location::zero() + Rows(5), Location::new(5, 0));
    assert_eq!(Location::zero() + Columns(-2), Location::new(0, -2));
    assert_eq!(Location::zero() + (2, 3), Location::new(2, 3));
    assert_eq!(
        Location::zero() + (Rows(1), Columns(1)),
        Location::new(1, 1)
    );
    assert_eq!(
        Location::zero() + (Columns(4), Rows(4)),
        Location::new(4, 4)
    );
    assert_eq!(Location::zero() + Up, Location::new(-1, 0));
}

impl<T: VectorLike> AddAssign<T> for Location {
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        let rhs = rhs.as_vector();

        self.row += rhs.rows;
        self.column += rhs.columns;
    }
}

#[cfg(test)]
#[test]
fn test_add_assign() {
    let mut loc = Location::zero();

    loc += Vector::new(-2, 5);
    assert_eq!(loc, Location::new(-2, 5));

    loc += Rows(4);
    assert_eq!(loc, Location::new(2, 5));

    loc += Columns(5);
    assert_eq!(loc, Location::new(2, 10));
}

impl<T: VectorLike> Sub<T> for Location {
    type Output = Location;

    #[inline]
    #[must_use]
    fn sub(self, rhs: T) -> Location {
        let rhs = rhs.as_vector();

        Location {
            row: self.row - rhs.rows,
            column: self.column - rhs.columns,
        }
    }
}

#[cfg(test)]
#[test]
fn test_sub() {
    assert_eq!(
        Location::new(3, 5) - Vector::new(-1, 6),
        Location::new(4, -1)
    );
    assert_eq!(Location::zero() - Rows(5), Location::new(-5, 0));
    assert_eq!(Location::zero() - Columns(-2), Location::new(0, 2));
    assert_eq!(Location::zero() - (2, 3), Location::new(-2, -3));
    assert_eq!(
        Location::zero() - (Rows(1), Columns(1)),
        Location::new(-1, -1)
    );
    assert_eq!(
        Location::zero() - (Columns(4), Rows(4)),
        Location::new(-4, -4)
    );
}

impl<T: VectorLike> SubAssign<T> for Location {
    #[inline]
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.as_vector();
        self.row -= rhs.rows;
        self.column -= rhs.columns;
    }
}

#[cfg(test)]
#[test]
fn test_sub_assign() {
    let mut loc = Location::zero();

    loc -= Vector::new(-2, 5);
    assert_eq!(loc, Location::new(2, -5));

    loc -= Rows(4);
    assert_eq!(loc, Location::new(-2, -5));

    loc -= Columns(5);
    assert_eq!(loc, Location::new(-2, -10));
}

// Note: we can't do Sub<LocationLike> because of the conflict with
// Sub<VectorLike>. Consider that (isize, isize) implements both VectorLike
// and LocationLike, which means that Location - (isize, isize) doesn't make
// sense anyway (though conceivably it could be resolved via return type
// analysis)
impl Sub<Location> for Location {
    type Output = Vector;

    #[inline]
    #[must_use]
    fn sub(self, rhs: Location) -> Vector {
        Vector {
            rows: self.row - rhs.row,
            columns: self.column - rhs.column,
        }
    }
}

impl Sub<(Row, Column)> for Location {
    type Output = Vector;

    #[inline]
    #[must_use]
    fn sub(self, (row, column): (Row, Column)) -> Vector {
        Vector {
            rows: self.row - row,
            columns: self.column - column,
        }
    }
}

impl Sub<(Column, Row)> for Location {
    type Output = Vector;

    #[inline]
    #[must_use]
    fn sub(self, (column, row): (Column, Row)) -> Vector {
        Vector {
            rows: self.row - row,
            columns: self.column - column,
        }
    }
}

// TODO: add other Sub<LocationLike> implementations as they're needed

#[cfg(test)]
#[test]
fn test_sub_self() {
    let loc1 = Location::new(4, 5);
    let loc2 = Location::new(1, 1);
    assert_eq!(loc1 - loc2, Vector::new(3, 4));
}

#[cfg(test)]
#[test]
fn test_sub_self_tuple() {
    let loc1 = Location::new(4, 5);
    let loc2 = (Row(1), Column(2));
    assert_eq!(loc1 - loc2, Vector::new(3, 3));
}

#[cfg(test)]
#[test]
fn test_sub_self_reverse_tuple() {
    let loc1 = Location::new(4, 5);
    let loc2 = (Column(2), Row(1));
    assert_eq!(loc1 - loc2, Vector::new(3, 3));
}

impl<T: LocationLike> PartialEq<T> for Location {
    #[must_use]
    #[inline]
    fn eq(&self, rhs: &T) -> bool {
        self.row == rhs.row() && self.column == rhs.column()
    }
}

/// Locations have a partial ordering. `loc1` is considered greater than `loc2` iff
/// its row or its column are greater than those in `loc2`. This chart shows an
/// example:
///
/// ```text
/// <<<??
/// <<<??
/// <<=>>
/// ??>>>
/// ??>>>
/// ```
///
/// Cells marked `>` are considered greater than the center location (marked `=`),
/// and cells marked '<' are less than the center location. Cells marked `?` do
/// not have an ordering with the center location.
///
/// For a strict ordering between all possible locations, see the [`Ordered`]
/// wrapper struct, which allows for row-major or column-major orderings.
impl<T: LocationLike> PartialOrd<T> for Location {
    #[must_use]
    fn partial_cmp(&self, rhs: &T) -> Option<Ordering> {
        match (self.row.cmp(&rhs.row()), self.column.cmp(&rhs.column())) {
            (Ordering::Greater, Ordering::Less) | (Ordering::Less, Ordering::Greater) => None,
            (o1, o2) => Some(o1.then(o2)),
        }
    }
}

#[cfg(test)]
mod partial_ord_tests {
    use crate::prelude::{Location, LocationLike};
    use crate::shorthand::{C, L, R};
    use core::cmp::Ordering;

    const ZERO: Location = Location::zero();

    #[test]
    fn test_eq() {
        assert_eq!(L(3, 4), (R(3), C(4)));
        assert_eq!(L(3, 4), (C(4), R(3)));
        assert_eq!(L(3, 4), L(3, 4));
        assert_eq!(L(3, 4), (3, 4));
    }

    #[test]
    fn test_orderliness() {
        assert_eq!(ZERO.partial_cmp(&ZERO.above(1)), Some(Ordering::Greater));
        assert_eq!(ZERO.partial_cmp(&ZERO.left(1)), Some(Ordering::Greater));
        assert_eq!(ZERO.partial_cmp(&(ZERO - (1, 1))), Some(Ordering::Greater));

        assert_eq!(ZERO.partial_cmp(&ZERO.below(1)), Some(Ordering::Less));
        assert_eq!(ZERO.partial_cmp(&ZERO.right(1)), Some(Ordering::Less));
        assert_eq!(ZERO.partial_cmp(&(ZERO + (1, 1))), Some(Ordering::Less));

        assert_eq!(ZERO.partial_cmp(&(ZERO + (-1, 1))), None);
        assert_eq!(ZERO.partial_cmp(&(ZERO - (-1, 1))), None);

        assert_eq!(ZERO.partial_cmp(&ZERO), Some(Ordering::Equal));
    }

    #[test]
    fn test_bad_diagonal() {
        for location in &[L(1, -1), L(-1, 1)] {
            assert!(!(ZERO < *location));
            assert!(!(ZERO > *location));
            assert!(!(ZERO <= *location));
            assert!(!(ZERO >= *location));
        }
    }
}

/// A pair of [`isize`] values acts as a `(`[`Row`]`, `[`Column`]`)` pair.
impl LocationLike for (isize, isize) {
    #[inline]
    #[must_use]
    fn row(&self) -> Row {
        Row(self.0)
    }

    #[inline]
    #[must_use]
    fn column(&self) -> Column {
        Column(self.1)
    }

    #[inline]
    #[must_use]
    fn as_location(&self) -> Location {
        Location::new(self.0, self.1)
    }
}

/// Rules for ordering a location. This struct wraps a [`LocationLike`] and
/// supplies an [`Ord`] and [`PartialOrd`] implementation. The `Major`
/// type parameter indicates which ordering is used; for instance,
/// `Ordering<Row>` provides row-major ordering, where Locations are sorted
/// first by row, then by column.
#[derive(Debug, Clone, Copy, Default, Hash)]
pub struct Ordered<L: LocationLike, Major: Component> {
    pub location: L,
    phantom: PhantomData<Major>,
}

impl<L: LocationLike, M: Component> Ordered<L, M> {
    #[inline]
    #[must_use]
    pub fn new(location: L) -> Self {
        Self {
            location,
            phantom: PhantomData,
        }
    }
}

impl<L: LocationLike, M: Component> From<L> for Ordered<L, M> {
    #[inline]
    #[must_use]
    fn from(location: L) -> Self {
        Self::new(location)
    }
}

impl<L: LocationLike, M: Component> AsRef<L> for Ordered<L, M> {
    #[inline]
    #[must_use]
    fn as_ref(&self) -> &L {
        &self.location
    }
}

impl<L: LocationLike, M: Component> AsMut<L> for Ordered<L, M> {
    #[inline]
    #[must_use]
    fn as_mut(&mut self) -> &mut L {
        &mut self.location
    }
}

impl<L: LocationLike, M: Component> Deref for Ordered<L, M> {
    type Target = L;

    #[inline]
    #[must_use]
    fn deref(&self) -> &L {
        &self.location
    }
}

impl<L: LocationLike, M: Component> DerefMut for Ordered<L, M> {
    #[inline]
    #[must_use]
    fn deref_mut(&mut self) -> &mut L {
        &mut self.location
    }
}

impl<L: LocationLike, M: Component> LocationLike for Ordered<L, M> {
    #[inline]
    #[must_use]
    fn row(&self) -> Row {
        self.location.row()
    }

    #[inline]
    #[must_use]
    fn column(&self) -> Column {
        self.location.column()
    }

    #[inline]
    #[must_use]
    fn as_location(&self) -> Location {
        self.location.as_location()
    }
}

impl<L: LocationLike, M: Component, R: LocationLike> PartialEq<R> for Ordered<L, M> {
    #[inline]
    #[must_use]
    fn eq(&self, rhs: &R) -> bool {
        self.as_location() == rhs.as_location()
    }
}

impl<L: LocationLike, M: Component> Eq for Ordered<L, M> {}

impl<L: LocationLike, M: Component> PartialOrd for Ordered<L, M> {
    #[inline]
    #[must_use]
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }

    #[inline]
    #[must_use]
    fn lt(&self, rhs: &Self) -> bool {
        self.cmp(rhs) == Ordering::Less
    }

    #[inline]
    #[must_use]
    fn le(&self, rhs: &Self) -> bool {
        self.cmp(rhs) != Ordering::Greater
    }

    #[inline]
    #[must_use]
    fn gt(&self, rhs: &Self) -> bool {
        self.cmp(rhs) == Ordering::Greater
    }

    #[inline]
    #[must_use]
    fn ge(&self, rhs: &Self) -> bool {
        self.cmp(rhs) != Ordering::Less
    }
}

impl<L: LocationLike, M: Component> Ord for Ordered<L, M> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        M::from_location(self)
            .cmp(&M::from_location(rhs))
            .then_with(move || {
                M::Converse::from_location(self).cmp(&M::Converse::from_location(rhs))
            })
    }
}

/// A generic type alias for ordering a [`LocationLike`] type `T` by row.
pub type RowOrdered<L> = Ordered<L, Row>;

/// A generic type for ordering a [`LocationLike`] type `T` by column.
pub type ColumnOrdered<L> = Ordered<L, Column>;

/// Type alias for a [`Location`] ordered by row.
pub type RowOrderedLocation = RowOrdered<Location>;

/// Type alias for a [`Location`] ordered by column.
pub type ColumnOrderedLocation = ColumnOrdered<Location>;
