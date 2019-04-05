use core::iter::FusedIterator;
use core::hash::Hash;
use core::ops::Range;
use core::fmt::Debug;
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::direction::*;
use crate::grid::{GridBounds, RangeError};
use crate::vector::{Columns, Component as VecComponent, Rows, Vector};
// TODO: add additional implied traits
// TODO: default?

/// A component of a [`Location`]
///
/// This trait comprises a component of a [`Location`], which may be either a
/// [`Row`] or a [`Column`]
pub trait Component:
    Sized +
    From<isize> +
    Copy +
    Debug +
    Ord +
    Eq +
    Hash
{
    /// The converse component ([`Row`] to [`Column`], or vice versa)
    type Converse: Component;

    /// The associated vector component ([`Rows`] or [`Columns`])
    type Distance: VecComponent;

    /// Get this component type from a [`Location`]
    fn from_location(location: &Location) -> Self;

    /// Combine this component with its converse to create a [`Location`]
    fn combine(self, other: Self::Converse) -> Location;

    /// Return the lowercase name of this component type– "row" or "column"
    fn name() -> &'static str;

    /// Runs grid.check_row or grid.check_column on this component
    fn check_against<G: GridBounds>(self, grid: &G) -> Result<Self, RangeError<Self>>;

    /// Get the integer value of this component
    fn value(self) -> isize;

    /// Add a distance to this component. This is provided as a method because
    /// Rust's trait system isn't powerful enough to allow for
    /// `Component: Add<Self::Distance>`
    fn add(self, amount: impl Into<Self::Distance>) -> Self;

    /// Find the distance between two components
    fn distance_to(self, target: Self) -> Self::Distance {
        target.distance_from(self)
    }

    fn distance_from(self, origin: Self) -> Self::Distance;
}

// TODO: add docstrings to these. Perhaps refer back to Component
macro_rules! make_component {
    (
        $Name:ident,
        $Converse:ident,
        $Distance:ident,
        $from_loc:ident,
        ($self:ident, $other:ident) =>
        ($first:ident, $second:ident),
        $name:expr,
        $in_bounds_method:ident
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

        impl<T: Into<$Distance>> Add<T> for $Name {
            type Output = $Name;

            fn add(self, rhs: T) -> Self {
                $Name(self.0 + rhs.into().0)
            }
        }

        impl<T: Into<$Distance>> AddAssign<T> for $Name {
            fn add_assign(&mut self, rhs: T) {
                self.0 += rhs.into().0
            }
        }

        impl<T: Into<$Distance>> Sub<T> for $Name {
            type Output = $Name;

            fn sub(self, rhs: T) -> Self {
                $Name(self.0 - rhs.into().0)
            }
        }

        impl<T: Into<$Distance>> SubAssign<T> for $Name {
            fn sub_assign(&mut self, rhs: T) {
                self.0 -= rhs.into().0
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

        impl From<$Name> for isize {
            fn from(value: $Name) -> isize {
                value.0
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

            fn check_against<G: GridBounds>(self, grid: &G) -> Result<Self, RangeError<Self>> {
                grid.$in_bounds_method(self)
            }

            fn add(self, distance: impl Into<Self::Distance>) -> Self {
                self + distance.into()
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

make_component! {Row,    Column, Rows,    row,    (self, other) => (self, other), "row",    check_row}
make_component! {Column, Row,    Columns, column, (self, other) => (other, self), "column", check_column}

// TODO: replace this with Range<C> once Step is stabilized
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ComponentRange<C: Component> {
    range: Range<isize>,
    phanton: PhantomData<C>,
}

impl<C: Component> ComponentRange<C> {
    pub fn new(start: C, end: C) -> Self {
        ComponentRange {
            phanton: PhantomData,
            range: start.value() .. end.value()
        }
    }

    pub fn span(start: C, size: C::Distance) -> Self {
        Self::new(start, start.add(size))
    }

    pub fn start(&self) -> C {
        self.range.start.into()
    }

    pub fn end(&self) -> C {
        self.range.end.into()
    }

    pub fn size(&self) -> C::Distance {
        self.start().distance_to(self.end())
    }
}

// TODO: add a bunch more iterator methods that forward to self.range.
impl<C: Component> Iterator for ComponentRange<C> {
    type Item = C;

    #[inline]
    fn next(&mut self) -> Option<C> {
        self.range.next().map(C::from)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<C> {
        self.range.nth(n).map(C::from)
    }

    #[inline]
    fn last(self) -> Option<C> {
        self.next_back()
    }

    #[inline]
    fn min(self) -> Option<C> {
        self.next()
    }

    #[inline]
    fn max(self) -> Option<C> {
        self.last()
    }
}

impl<C: Component> DoubleEndedIterator for ComponentRange<C> {
    fn next_back(&mut self) -> Option<C> {
        self.range.next_back().map(C::from)
    }
}

impl<C: Component> ExactSizeIterator for ComponentRange<C> {}
impl<C: Component> FusedIterator for ComponentRange<C> {}
// TODO: TrustedLen

pub type RowRange = ComponentRange<Row>;
pub type ColumnRange = ComponentRange<Column>;

/// A location on a grid
///
/// A location is the primary indexing type for a Grid, and represents a single
/// cell on that grid. It is comprised of a Row and a Column. By convention,
/// increasing rows count downward and increasing columns count rightward.
///
/// Locations support arithmetic operations with Vectors.
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct Location {
    pub row: Row,
    pub column: Column,
}

impl Location {
    pub fn new(row: impl Into<Row>, column: impl Into<Column>) -> Self {
        Location {
            row: row.into(),
            column: column.into(),
        }
    }

    pub fn get_component<T: Component>(&self) -> T {
        T::from_location(self)
    }

    pub fn above(&self, distance: impl Into<Rows>) -> Location {
        *self - distance.into()
    }

    pub fn below(&self, distance: impl Into<Rows>) -> Location {
        *self + distance.into()
    }

    pub fn left(&self, distance: impl Into<Columns>) -> Location {
        *self - distance.into()
    }

    pub fn right(&self, distance: impl Into<Columns>) -> Location {
        *self + distance.into()
    }

    pub fn relative(&self, direction: Direction, distance: isize) -> Location {
        *self + Vector::in_direction(direction, distance)
    }
}

impl<R: Into<Row>, C: Into<Column>> From<(R, C)> for Location {
    fn from(value: (R, C)) -> Location {
        Location::new(value.0, value.1)
    }
}

impl<T: Into<Vector>> Add<T> for Location {
    type Output = Location;

    fn add(self, rhs: T) -> Location {
        let rhs = rhs.into();
        Location::new(self.row + rhs.rows, self.column + rhs.columns)
    }
}

impl<T: Into<Vector>> AddAssign<T> for Location {
    fn add_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.row += rhs.rows;
        self.column += rhs.columns;
    }
}

impl<T: Into<Vector>> Sub<T> for Location {
    type Output = Location;

    fn sub(self, rhs: T) -> Location {
        let rhs = rhs.into();
        Location::new(self.row - rhs.rows, self.column - rhs.columns)
    }
}

impl<T: Into<Vector>> SubAssign<T> for Location {
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.row -= rhs.rows;
        self.column -= rhs.columns;
    }
}

impl Sub<Location> for Location {
    type Output = Vector;

    fn sub(self, rhs: Location) -> Vector {
        Vector::new(self.row - rhs.row, self.column - rhs.column)
    }
}
