use core::cmp::{Ordering, PartialOrd};
use core::fmt::Debug;
use core::hash::Hash;
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Deref, DerefMut, Sub, SubAssign};

use crate::direction::Direction;
use crate::vector::{Columns, Component as VecComponent, Rows, Vector};

// TODO: add additional implied traits?
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
    fn from_location(location: &Location) -> Self;

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
    fn name() -> &'static str;

    /// Get the integer value of this component
    ///
    /// ```
    /// use gridly::prelude::*;
    ///
    /// assert_eq!(Row(5).value(), 5);
    /// ```
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
    fn add_distance(self, amount: Self::Distance) -> Self;

    /// Find the distance between two components, using this component as the origin
    ///
    /// ```
    /// use gridly::prelude::*;
    ///
    /// assert_eq!(Row(3).distance_to(Row(8)), Rows(5));
    /// ```
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
    fn distance_from(self, origin: Self) -> Self::Distance;

    /// Convert a Row into a Column or vice versa
    ///
    /// /// ```
    /// use gridly::prelude::*;
    ///
    /// assert_eq!(Row(3).transpose(), Column(3));
    /// ```
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
        $name:literal,

        // The name of the module in which to place unit tests
        $test:ident
    ) => {
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[doc = "A `"]
        #[doc = $name]
        #[doc = "` component of a [`Location`]"]
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

            #[inline]
            fn combine($self, $other: Self::Converse) -> Location {
                Location::new($first, $second)
            }

            #[inline(always)]
            fn name() -> &'static str {
                $name
            }

            #[inline]
            fn add_distance(self, distance: Self::Distance) -> Self {
                self + distance
            }

            #[inline]
            fn distance_from(self, origin: Self) -> Self::Distance {
                self - origin
            }

            #[inline]
            fn value(self) -> isize {
                self.0
            }
        }

        #[cfg(test)]
        mod $test {
            use crate::location::{Location, $Name, $Converse, Component};
            use crate::vector::{$Distance};

            #[test]
            fn test_add_converse() {
                let base = $Name(3);
                let converse = $Converse(4);

                assert_eq!(base + converse, base.combine(converse));
                assert_eq!($Name(0) + $Converse(0), Location::new(0, 0))
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
    }
}

make_component! {Row,    Column, Rows,    row,    (self, other) => (self, other), "row",    test_row}
make_component! {Column, Row,    Columns, column, (self, other) => (other, self), "column", test_column}

/// A location on a grid
///
/// A location is the primary indexing type for a Grid, and represents a single
/// cell on that grid. It is comprised of a Row and a Column. Increasing rows
/// count downward and increasing columns count rightward.
///
/// Locations support arithmetic operations with Vectors.
#[derive(Debug, Clone, Copy, Default, Hash, Eq)]
pub struct Location {
    pub row: Row,
    pub column: Column,
}

impl Location {
    /// Create a new location out of a `row` and a `column`
    #[inline]
    pub fn new(row: impl Into<Row>, column: impl Into<Column>) -> Self {
        Location {
            row: row.into(),
            column: column.into(),
        }
    }

    /// Create a new location at `(0, 0)`.
    pub const fn zero() -> Self {
        Location {
            row: Row(0),
            column: Column(0),
        }
    }

    /// Get either the row or column of a location. This method is useful in
    /// code that is generic over the Row or Column.
    #[inline]
    pub fn get_component<T: Component>(&self) -> T {
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
    pub fn above(&self, distance: impl Into<Rows>) -> Location {
        *self - distance.into()
    }

    /// Return the location that is `distance` rows below this one
    ///
    /// ```
    /// use gridly::prelude::*;
    /// use gridly::shorthand::*;
    ///
    /// assert_eq!(L(3, 4).below(2), L(5, 4));
    /// ```
    pub fn below(&self, distance: impl Into<Rows>) -> Location {
        *self + distance.into()
    }

    /// Return the location that is `distance` columns to the left of this one
    ///
    /// ```
    /// use gridly::prelude::*;
    /// use gridly::shorthand::*;
    ///
    /// assert_eq!(L(3, 4).left(2), L(3, 2));
    /// ```
    pub fn left(&self, distance: impl Into<Columns>) -> Location {
        *self - distance.into()
    }

    /// Return the location that is `distance` columns to the right of this one
    ///
    /// ```
    /// use gridly::prelude::*;
    /// use gridly::shorthand::*;
    ///
    /// assert_eq!(L(3, 4).right(2), L(3, 6));
    /// ```
    pub fn right(&self, distance: impl Into<Columns>) -> Location {
        *self + distance.into()
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
    pub fn relative(&self, direction: Direction, distance: isize) -> Location {
        *self + (direction * distance)
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
    pub fn step(&self, direction: Direction) -> Location {
        *self + direction
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
    pub fn transpose(&self) -> Location {
        Location::new(self.column.transpose(), self.row.transpose())
    }

    /// Generically get strictly ordered version of this `Location`. The `Major`
    /// is the ordering; for example, `order_by::<Row>` will create a row-ordered
    /// `Location`. See [`row_ordered`] or [`column_ordered`] for an example.
    #[inline]
    pub fn order_by<Major: Component>(self) -> OrderedLocation<Major> {
        self.into()
    }

    /// Get a strictly row-ordered version of this `Location`; that is, a location
    /// which is ordered by comparing the `row`, then the `columns`.
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
    pub fn row_ordered(self) -> RowOrderedLocation {
        self.order_by()
    }

    //// Get a strictly row-ordered version of this `Location`; that is, a location
    /// which is ordered by comparing the `row`, then the `columns`.
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
    pub fn column_ordered(self) -> ColumnOrderedLocation {
        self.order_by()
    }
}

impl From<(Row, Column)> for Location {
    fn from((row, column): (Row, Column)) -> Location {
        Location::new(row, column)
    }
}

impl From<(Column, Row)> for Location {
    fn from((column, row): (Column, Row)) -> Location {
        Location::new(row, column)
    }
}

impl From<(isize, isize)> for Location {
    fn from((row, column): (isize, isize)) -> Location {
        Location::new(row, column)
    }
}

impl<T: Into<Vector>> Add<T> for Location {
    type Output = Location;

    fn add(self, rhs: T) -> Location {
        let rhs = rhs.into();
        Location::new(self.row + rhs.rows, self.column + rhs.columns)
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

impl<T: Into<Vector>> AddAssign<T> for Location {
    fn add_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
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

impl<T: Into<Vector>> Sub<T> for Location {
    type Output = Location;

    fn sub(self, rhs: T) -> Location {
        let rhs = rhs.into();
        Location::new(self.row - rhs.rows, self.column - rhs.columns)
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

impl<T: Into<Vector>> SubAssign<T> for Location {
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
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

impl Sub<Location> for Location {
    type Output = Vector;

    fn sub(self, rhs: Location) -> Vector {
        Vector::new(self.row - rhs.row, self.column - rhs.column)
    }
}

#[cfg(test)]
#[test]
fn test_sub_self() {
    let loc1 = Location::new(4, 5);
    let loc2 = Location::new(1, 1);
    assert_eq!(loc1 - loc2, Vector::new(3, 4));
}

impl<T: Into<Location> + Copy> PartialEq<T> for Location {
    fn eq(&self, rhs: &T) -> bool {
        let rhs = (*rhs).into();
        self.row == rhs.row && self.column == rhs.column
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
/// For a strict ordering between all possible locations, see the [`OrderedLocation`]
/// wrapper struct, which allows for row-major or column-major orderings.
impl<T: Into<Location> + Copy> PartialOrd<T> for Location {
    fn partial_cmp(&self, rhs: &T) -> Option<Ordering> {
        let rhs = (*rhs).into();
        match (self.row.cmp(&rhs.row), self.column.cmp(&rhs.column)) {
            (Ordering::Greater, Ordering::Less) | (Ordering::Less, Ordering::Greater) => None,
            (o1, o2) => Some(o1.then(o2)),
        }
    }
}

#[cfg(test)]
mod partial_ord_tests {
    use crate::prelude::Location;
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

/// Rules for ordering a Location
///
/// `OrderedLocation` is a wrapper struct around a `Location` that supplies an Ord and
/// PartialOrd implementation. The `Major` type parameter indicates which
/// ordering is used; for instance, `Ordering<Row>` provides row-major ordering,
/// where Locations are sorted first by row, then by column.
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct OrderedLocation<Major: Component> {
    pub location: Location,
    phantom: PhantomData<Major>,
}

impl<M: Component> OrderedLocation<M> {
    #[inline]
    pub fn new(location: Location) -> Self {
        Self {
            location,
            phantom: PhantomData,
        }
    }
}

impl<M: Component> From<OrderedLocation<M>> for Location {
    #[inline]
    fn from(ord: OrderedLocation<M>) -> Self {
        ord.location
    }
}

impl<M: Component> From<Location> for OrderedLocation<M> {
    #[inline]
    fn from(location: Location) -> Self {
        Self::new(location)
    }
}

impl<M: Component> AsRef<Location> for OrderedLocation<M> {
    #[inline]
    fn as_ref(&self) -> &Location {
        &self.location
    }
}

impl<M: Component> AsMut<Location> for OrderedLocation<M> {
    #[inline]
    fn as_mut(&mut self) -> &mut Location {
        &mut self.location
    }
}

impl<M: Component> Deref for OrderedLocation<M> {
    type Target = Location;

    #[inline]
    fn deref(&self) -> &Location {
        &self.location
    }
}

impl<M: Component> DerefMut for OrderedLocation<M> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Location {
        &mut self.location
    }
}

impl<M: Component> PartialOrd for OrderedLocation<M> {
    #[inline]
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }

    #[inline]
    fn lt(&self, rhs: &Self) -> bool {
        self.cmp(rhs) == Ordering::Less
    }

    #[inline]
    fn le(&self, rhs: &Self) -> bool {
        self.cmp(rhs) != Ordering::Greater
    }

    #[inline]
    fn gt(&self, rhs: &Self) -> bool {
        self.cmp(rhs) == Ordering::Greater
    }

    #[inline]
    fn ge(&self, rhs: &Self) -> bool {
        self.cmp(rhs) != Ordering::Less
    }
}

impl<M: Component> Ord for OrderedLocation<M> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        M::from_location(self)
            .cmp(&M::from_location(rhs))
            .then_with(move || {
                M::Converse::from_location(self).cmp(&M::Converse::from_location(rhs))
            })
    }
}

pub type RowOrderedLocation = OrderedLocation<Row>;
pub type ColumnOrderedLocation = OrderedLocation<Column>;
