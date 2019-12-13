//! Range types, similar to [`core::ops::Range`], for easy iteration over
//! [`Row`], [`Column`], and [`Location`] values.
//!
//! [`Row`]: crate::location::Row
//! [`Column`]: crate::location::Column
//! [`Location`]: crate::location::Location

use core::cmp::Ordering;
use core::fmt::{self, Display, Formatter};
use core::iter::FusedIterator;
use core::marker::PhantomData;
use core::ops::Range;

use crate::location::{Column, Component, Location, LocationLike, Row};
use crate::vector::Component as VecComponent;

// TODO: replace this with ops::Range<C> once Step is stabilized. Mostly
// we want this so that we can take advantage of `Range`'s very optimized
// iterators.
/// A range over [`Row`] or [`Column`] values. Much like the standard rust
/// [`Range`](::core::ops::Range), it is half open, bounded by `[start..end)`.
/// It supports simple accessors and iteration. It also forms the basis
/// for bounds checking, through the [`check`][ComponentRange::check] method.
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
    #[must_use]
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
    #[must_use]
    #[inline]
    pub fn span(start: C, size: C::Distance) -> Self {
        Self::bounded(start, start.add_distance(size))
    }

    /// Get the start index of the range
    #[must_use]
    #[inline]
    pub fn start(&self) -> C {
        self.range.start.into()
    }

    /// Get the end index of the range
    #[must_use]
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
    #[must_use]
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
    #[must_use]
    #[inline]
    pub fn in_bounds(&self, loc: impl Into<C>) -> bool {
        self.check(loc).is_ok()
    }

    /// Combine an index range with a converse index to create a [`LocationRange`]
    ///
    /// # Example:
    ///
    /// ```
    /// use gridly::range::RowRange;
    /// use gridly::location::{Row, Column};
    /// use gridly::shorthand::L;
    ///
    /// let row_range = RowRange::bounded(Row(3), Row(6));
    /// let mut loc_range = row_range.cross(Column(4));
    ///
    /// assert_eq!(loc_range.next(), Some(L(3, 4)));
    /// assert_eq!(loc_range.next(), Some(L(4, 4)));
    /// assert_eq!(loc_range.next(), Some(L(5, 4)));
    /// assert_eq!(loc_range.next(), None);
    /// ```
    #[must_use]
    #[inline]
    pub fn cross(self, index: C::Converse) -> LocationRange<C::Converse> {
        LocationRange::new(index, self)
    }
}

// TODO: impl RangeBounds for ComponentRange.

// TODO: add a bunch more iterator methods that forward to self.range;
impl<C: Component> Iterator for ComponentRange<C> {
    type Item = C;

    #[inline]
    fn next(&mut self) -> Option<C> {
        self.range.next().map(C::from)
    }

    #[must_use]
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
            RangeError::TooLow(min) => write!(f, "{} too low, must be >= {:?}", T::name(), min),
            RangeError::TooHigh(max) => write!(f, "{} too high, must be < {:?}", T::name(), max),
        }
    }
}

// TODO: Add this when we figure out how to make it compatible with no_std
/* impl<T: Component> Error for Component {} */

pub type RowRangeError = RangeError<Row>;
pub type ColumnRangeError = RangeError<Column>;

/// A range over [`Location`]s in a given [`Row`] or [`Column`].
///
/// The generic parameter is the direction of the range; that is to say, a
/// `LocationRange<Row>` is a range of locations in a given row. Each location
/// in the range has the same `row` but a different `column`.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct LocationRange<C: Component> {
    index: C,
    range: ComponentRange<C::Converse>,
}

impl<C: Component> LocationRange<C> {
    #[inline]
    #[must_use]
    pub fn new(index: C, range: ComponentRange<C::Converse>) -> Self {
        LocationRange { index, range }
    }

    #[inline]
    #[must_use]
    pub fn bounded(index: C, start: C::Converse, end: C::Converse) -> Self {
        Self::new(index, ComponentRange::bounded(start, end))
    }

    #[inline]
    #[must_use]
    pub fn span(index: C, start: C::Converse, size: <C::Converse as Component>::Distance) -> Self {
        Self::new(index, ComponentRange::span(start, size))
    }

    #[inline]
    #[must_use]
    pub fn rooted(root: Location, size: <C::Converse as Component>::Distance) -> Self {
        Self::span(root.get_component(), root.get_component(), size)
    }

    #[inline]
    #[must_use]
    pub fn component_range(&self) -> ComponentRange<C::Converse> {
        self.range.clone()
    }

    #[inline]
    #[must_use]
    pub fn index(&self) -> C {
        self.index
    }

    #[inline]
    #[must_use]
    pub fn start(&self) -> Location {
        self.range.start().combine(self.index)
    }

    #[inline]
    #[must_use]
    pub fn end(&self) -> Location {
        self.range.end().combine(self.index)
    }

    #[inline]
    #[must_use]
    pub fn size(&self) -> <C::Converse as Component>::Distance {
        self.range.start().distance_to(self.range.end())
    }
}

impl LocationRange<Row> {
    #[inline]
    #[must_use]
    pub fn row(&self) -> Row {
        self.index
    }

    #[inline]
    #[must_use]
    pub fn columns(&self) -> ColumnRange {
        self.component_range()
    }
}

impl LocationRange<Column> {
    #[inline]
    #[must_use]
    pub fn column(&self) -> Column {
        self.index
    }

    #[inline]
    #[must_use]
    pub fn rows(&self) -> RowRange {
        self.component_range()
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

    #[must_use]
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

/// A range over `Locations`, in row or column-major order.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CrossRange<C: Component> {
    // The nomenclature here assume row-major, to aid in readability.

    // Our current top component
    top: C,

    // Our current bottom component. Inclusive; may == top.
    bottom: C,

    // This is the span we reset to each time we step top or bottom
    span: ComponentRange<C::Converse>,

    next_front: C::Converse,

    // Exclusive outer bound
    next_back: C::Converse,
}

impl<C: Component> CrossRange<C> {
    #[must_use]
    #[inline]
    pub fn new(major: ComponentRange<C>, cross: ComponentRange<C::Converse>) -> Self {
        Self {
            top: major.start(),
            bottom: major.end().add_distance(-1),

            next_front: cross.start(),
            next_back: cross.end(),

            span: cross,
        }
    }

    /// Count the remianing elements in this iterator. Helper for size_hint.
    #[must_use]
    fn size(&self) -> Option<usize> {
        match self.top.cmp(&self.bottom) {
            Ordering::Greater => Some(0),
            Ordering::Equal => Some(self.next_front.distance_to(self.next_back).value() as usize),
            Ordering::Less => {
                // Safety: all of the ranges here are guaranteed to be positive or zero
                // by the contract of this struct. top < bottom, so the -1 is safe.
                let hamburger_thickness = (self.top.distance_to(self.bottom).value() as usize) - 1;
                let hamburger_size =
                    hamburger_thickness.checked_mul(self.span.size().value() as usize)?;

                let top_bun = self.next_front.distance_to(self.span.end()).value() as usize;
                let bottom_bun = self.span.start().distance_to(self.next_back).value() as usize;

                hamburger_size.checked_add(top_bun)?.checked_add(bottom_bun)
            }
        }
    }
}

impl<C: Component> Iterator for CrossRange<C> {
    type Item = Location;

    fn next(&mut self) -> Option<Location> {
        loop {
            match self.top.cmp(&self.bottom) {
                Ordering::Greater => break None,
                Ordering::Equal if self.next_front >= self.next_back => break None,
                Ordering::Less if self.next_front >= self.span.end() => {
                    self.top = self.top.add_distance(1);
                    self.next_front = self.span.start();
                }
                _ => {
                    let index = self.next_front;
                    self.next_front = self.next_front.add_distance(1);
                    break Some(index.combine(self.top));
                }
            }
        }
    }

    #[must_use]
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.size();

        (size.unwrap_or(core::usize::MAX), size)
    }
}

impl<C: Component> DoubleEndedIterator for CrossRange<C> {
    fn next_back(&mut self) -> Option<Location> {
        loop {
            match self.top.cmp(&self.bottom) {
                Ordering::Greater => break None,
                Ordering::Equal if self.next_front >= self.next_back => break None,
                Ordering::Less if self.next_back <= self.span.start() => {
                    self.bottom = self.bottom.add_distance(-1);
                    self.next_back = self.span.end();
                }
                _ => {
                    self.next_back = self.next_back.add_distance(-1);
                    break Some(self.next_back.combine(self.bottom));
                }
            }
        }
    }
}

impl<C: Component> FusedIterator for CrossRange<C> {}
impl<C: Component> ExactSizeIterator for CrossRange<C> {}

#[test]
fn test_cross_range() {
    use crate::vector::{Columns, Rows};

    let row_range = RowRange::span(Row(-2), Rows(5));
    let column_range = ColumnRange::span(Column(3), Columns(4));

    let mut cross_range = CrossRange::new(row_range.clone(), column_range.clone());

    let mut remaining = 20;

    assert_eq!(cross_range.size_hint(), (20, Some(20)));

    for row in row_range {
        for column in column_range.clone() {
            let actual = cross_range.next();
            remaining -= 1;

            assert_eq!(Some(row + column), actual);
            assert_eq!(cross_range.size_hint(), (remaining, Some(remaining)));
        }
    }

    assert_eq!(cross_range.next(), None);
}

#[test]
fn test_cross_range_reverse() {
    use crate::vector::{Columns, Rows};

    let row_range = RowRange::span(Row(-2), Rows(5));
    let column_range = ColumnRange::span(Column(3), Columns(4));

    let mut cross_range = CrossRange::new(row_range.clone(), column_range.clone());

    let mut remaining = 20;

    assert_eq!(cross_range.size_hint(), (20, Some(20)));

    for row in row_range.rev() {
        for column in column_range.clone().rev() {
            let actual = cross_range.next_back();
            remaining -= 1;

            assert_eq!(Some(row + column), actual);
            assert_eq!(cross_range.size_hint(), (remaining, Some(remaining)));
        }
    }

    assert_eq!(cross_range.next_back(), None);
}

/// Test that iteration and size hint are correct even when the iterator
/// is being iterated from both ends.
#[test]
fn test_cross_range_converge() {
    use crate::vector::{Columns, Rows};

    let row_range = RowRange::span(Row(-2), Rows(5));
    let column_range = ColumnRange::span(Column(3), Columns(4));

    let mut converging_cross_range = CrossRange::new(row_range, column_range);

    let mut front_cross_range = converging_cross_range.clone();
    let mut back_cross_range = converging_cross_range.clone();

    let mut remaining = 20;

    while remaining > 0 {
        let next_front = front_cross_range.next();
        let converge_front = converging_cross_range.next();
        remaining -= 1;

        assert!(converge_front.is_some());
        assert_eq!(next_front, converge_front);
        assert_eq!(
            converging_cross_range.size_hint(),
            (remaining, Some(remaining))
        );

        let next_back = back_cross_range.next_back();
        let converge_back = converging_cross_range.next_back();
        remaining -= 1;

        assert!(converge_back.is_some());
        assert_eq!(next_back, converge_back);
        assert_eq!(
            converging_cross_range.size_hint(),
            (remaining, Some(remaining))
        );
    }

    assert_eq!(converging_cross_range.size_hint(), (0, Some(0)));
    assert_eq!(converging_cross_range.next(), None);
    assert_eq!(converging_cross_range.next_back(), None);
    assert_eq!(converging_cross_range.size_hint(), (0, Some(0)));
}
