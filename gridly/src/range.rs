use core::fmt::{self, Display, Formatter};
use core::iter::FusedIterator;
use core::marker::PhantomData;
use core::ops::Range;

use crate::location::{Column, Component, Location, Row, LocationLike};

// TODO: replace this with ops::Range<C> once Step is stabilized. Mostly
// we want this so that we can take advantage of `Range`'s very optimized
// iterators.
/// Range over `Row` or `Column` indices.
///
/// This struct represents a range over `Row` or `Column` values. Much like
/// the standard [Rust `Range`](::core::ops::Range), it is half open, bounded
/// by `[start..end)`. It supports simple accessors and iteration.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ComponentRange<C: Component> {
    range: Range<isize>,
    phanton: PhantomData<C>,
}

impl<C: Component> ComponentRange<C> {
    /// Create a range bounded by `[start .. end)`.
    ///
    /// # Example:
    ///
    /// ```
    /// use gridly::range::ComponentRange;
    /// use gridly::location::Row;
    /// use gridly::vector::Rows;
    ///
    /// let mut range = ComponentRange::bounded(Row(0), Row(3));
    ///
    /// assert_eq!(range.size(), Rows(3));
    /// assert_eq!(range.start(), Row(0));
    /// assert_eq!(range.end(), Row(3));
    ///
    /// assert_eq!(range.next(), Some(Row(0)));
    /// assert_eq!(range.next(), Some(Row(1)));
    /// assert_eq!(range.next(), Some(Row(2)));
    /// assert_eq!(range.next(), None);
    /// ```
    #[inline]
    pub fn bounded(start: C, end: C) -> Self {
        ComponentRange {
            phanton: PhantomData,
            range: start.value()..end.value(),
        }
    }

    /// Create a range starting at `start` with length `size`
    ///
    /// # Example:
    ///
    /// ```
    /// use gridly::range::ComponentRange;
    /// use gridly::location::Column;
    /// use gridly::vector::Columns;
    ///
    /// let mut range = ComponentRange::span(Column(1), Columns(2));
    ///
    /// assert_eq!(range.size(), Columns(2));
    /// assert_eq!(range.start(), Column(1));
    /// assert_eq!(range.end(), Column(3));
    ///
    /// assert_eq!(range.next(), Some(Column(1)));
    /// assert_eq!(range.next(), Some(Column(2)));
    /// assert_eq!(range.next(), None);
    /// ```
    #[inline]
    pub fn span(start: C, size: C::Distance) -> Self {
        Self::bounded(start, start.add_distance(size))
    }

    /// Get the start index of the range
    #[inline]
    pub fn start(&self) -> C {
        self.range.start.into()
    }

    /// Get the end index of the range
    #[inline]
    pub fn end(&self) -> C {
        self.range.end.into()
    }

    /// Get the size of the range
    ///
    /// # Example:
    ///
    /// ```
    /// use gridly::range::ComponentRange;
    /// use gridly::location::Row;
    /// use gridly::vector::Rows;
    ///
    /// let range = ComponentRange::bounded(Row(-1), Row(3));
    /// assert_eq!(range.size(), Rows(4));
    /// ```
    #[inline]
    pub fn size(&self) -> C::Distance {
        self.start().distance_to(self.end())
    }

    /// Check that a `Row` or `Column` is in bounds for this range. If it is,
    /// return the index as a `Row` or `Column`; otherwise, return a `RangeError`
    /// indivating if the index was too high or too low, and what the exceeded
    /// upper or lower bound is.
    ///
    /// # Example:
    ///
    /// ```
    /// use gridly::range::{ComponentRange, RangeError};
    /// use gridly::location::Row;
    /// use gridly::vector::Rows;
    ///
    /// let range = ComponentRange::span(Row(3), Rows(5));
    /// assert_eq!(range.check(Row(4)), Ok(Row(4)));
    /// assert_eq!(range.check(Row(0)), Err(RangeError::TooLow(Row(3))));
    /// assert_eq!(range.check(Row(8)), Err(RangeError::TooHigh(Row(8))));
    ///
    /// // This can also be used to quickly map an `isize` to a `Row` or `Column`
    /// assert_eq!(range.check(5), Ok(Row(5)));
    /// ```
    pub fn check(&self, idx: impl Into<C>) -> Result<C, RangeError<C>> {
        let idx = idx.into();

        let min = self.start();
        let max = self.end();

        if idx < min {
            Err(RangeError::TooLow(min))
        } else if idx >= max {
            Err(RangeError::TooHigh(max))
        } else {
            Ok(idx)
        }
    }

    /// Check that a `Row` or `Column` is in bounds for this range.
    #[inline]
    pub fn in_bounds(&self, loc: impl Into<C>) -> bool {
        self.check(loc).is_ok()
    }

    /// Combine an index range with a converse index to create a [location range],
    /// which is a range over locations, rather than Row or Column indexes.
    ///
    /// # Example:
    ///
    /// ```
    /// use gridly::range::ComponentRange;
    /// use gridly::location::{Row, Column};
    /// use gridly::shorthand::L;
    ///
    /// let mut range = ComponentRange::bounded(Row(3), Row(6)).combine(Column(4));
    ///
    /// assert_eq!(range.next(), Some(L(3, 4)));
    /// assert_eq!(range.next(), Some(L(4, 4)));
    /// assert_eq!(range.next(), Some(L(5, 4)));
    /// assert_eq!(range.next(), None);
    /// ```
    pub fn combine(self, index: C::Converse) -> LocationRange<C::Converse> {
        LocationRange::new(index, self)
    }
}

// TODO: impl RangeBounds for ComponentRange.

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
    fn last(mut self) -> Option<C> {
        self.next_back()
    }
}

impl<C: Component> DoubleEndedIterator for ComponentRange<C> {
    #[inline]
    fn next_back(&mut self) -> Option<C> {
        self.range.next_back().map(C::from)
    }
}

impl<C: Component> ExactSizeIterator for ComponentRange<C> {}
impl<C: Component> FusedIterator for ComponentRange<C> {}
// TODO: TrustedLen when stable

pub type RowRange = ComponentRange<Row>;
pub type ColumnRange = ComponentRange<Column>;

// TODO: Error implementation

/// Error indicating that a Row or Column was out of bounds.
///
/// Note that the bounds expressed in this error are half inclusive; that is,
/// the lower bound in TooLow is an inclusive lower bound, but the upper bound
/// in TooHigh is an exclusive upper bound. This is consistent with the
/// conventional range representation of `low..high`.
///
/// # Example:
///
/// ```
/// use gridly::range::{ComponentRange, RangeError};
/// use gridly::location::Row;
/// let range = ComponentRange::bounded(Row(0), Row(10));
///
/// assert_eq!(range.check(-5), Err(RangeError::TooLow(Row(0))));
/// assert_eq!(range.check(15), Err(RangeError::TooHigh(Row(10))));
/// assert_eq!(range.check(10), Err(RangeError::TooHigh(Row(10))));
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RangeError<T: Component> {
    /// The given row or column was too low. The value in the error is the
    /// minimum row or column, inclusive.
    TooLow(T),

    /// The given row or column was too high. The given value in the error is
    /// the maximum row or column, exclusive (that is, a value *equal* to the
    /// error value is considered too high).
    TooHigh(T),
}

impl<T: Component> Display for RangeError<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            RangeError::TooLow(min) => write!(f, "too low, must be >= {:?}", min),
            RangeError::TooHigh(max) => write!(f, "too high, must be < {:?}", max),
        }
    }
}

// TODO: Add this when we figure out how to make it compatible with no_std
/* impl<T: Component> Error for Component {} */

pub type RowRangeError = RangeError<Row>;
pub type ColumnRangeError = RangeError<Column>;

/// A range over [`Location`]s in a given [`Row`] or [`Column`]. The generic
/// parameter is the direction of the range; that is to say, a
/// `LocationRange<Row>` is a range of locations in a given row. Each location
/// in the range has the same `row` but a different `column`.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct LocationRange<C: Component> {
    index: C,
    range: ComponentRange<C::Converse>,
}

impl<C: Component> LocationRange<C> {
    #[inline]
    pub fn new(index: C, range: ComponentRange<C::Converse>) -> Self {
        LocationRange { index, range }
    }

    #[inline]
    pub fn bounded(index: C, start: C::Converse, end: C::Converse) -> Self {
        Self::new(index, ComponentRange::bounded(start, end))
    }

    #[inline]
    pub fn span(index: C, start: C::Converse, size: <C::Converse as Component>::Distance) -> Self {
        Self::bounded(index, start, start.add_distance(size))
    }

    #[inline]
    pub fn rooted(root: Location, size: <C::Converse as Component>::Distance) -> Self {
        Self::span(root.get_component(), root.get_component(), size)
    }

    #[inline]
    pub fn component_range(&self) -> ComponentRange<C::Converse> {
        self.range.clone()
    }

    #[inline]
    pub fn index(&self) -> C {
        self.index
    }

    pub fn start(&self) -> Location {
        self.range.start().combine(self.index)
    }

    pub fn end(&self) -> Location {
        self.range.end().combine(self.index)
    }

    pub fn size(&self) -> <C::Converse as Component>::Distance {
        self.range.start().distance_to(self.range.end())
    }
}

impl<C: Component> Iterator for LocationRange<C> {
    type Item = Location;

    #[inline]
    fn next(&mut self) -> Option<Location> {
        self.range
            .next()
            .map(move |cross| cross.combine(self.index))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Location> {
        self.range
            .nth(n)
            .map(move |cross| cross.combine(self.index))
    }

    #[inline]
    fn last(self) -> Option<Location> {
        let index = self.index;
        self.range.last().map(move |cross| cross.combine(index))
    }
}

impl<C: Component> DoubleEndedIterator for LocationRange<C> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range
            .next_back()
            .map(move |cross| cross.combine(self.index))
    }
}

impl<C: Component> FusedIterator for LocationRange<C> {}
impl<C: Component> ExactSizeIterator for LocationRange<C> {}
// TODO: TrustedLen
