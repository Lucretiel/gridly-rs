use core::iter::FusedIterator;
use core::marker::PhantomData;
use core::ops::Range as IRange;

use crate::location::{self, Column, Component, Row};

// TODO: replace this with Range<C> once Step is stabilized
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Range<C: Component> {
    range: IRange<isize>,
    phanton: PhantomData<C>,
}

impl<C: Component> Range<C> {
    pub fn bounded(start: C, end: C) -> Self {
        Range {
            phanton: PhantomData,
            range: start.value()..end.value(),
        }
    }

    #[inline]
    pub fn span(start: C, size: C::Distance) -> Self {
        Self::bounded(start, start.add(size))
    }

    #[inline]
    pub fn start(&self) -> C {
        self.range.start.into()
    }

    #[inline]
    pub fn end(&self) -> C {
        self.range.end.into()
    }

    #[inline]
    pub fn size(&self) -> C::Distance {
        self.start().distance_to(self.end())
    }

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

    #[inline]
    pub fn in_bounds(&self, loc: C) -> bool {
        self.check(loc).is_ok()
    }

    pub fn combine(self, index: C::Converse) -> location::Range<C::Converse> {
        location::Range::new(index, self)
    }
}

// TODO: add a bunch more iterator methods that forward to self.range.
impl<C: Component> Iterator for Range<C> {
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

impl<C: Component> DoubleEndedIterator for Range<C> {
    fn next_back(&mut self) -> Option<C> {
        self.range.next_back().map(C::from)
    }
}

impl<C: Component> ExactSizeIterator for Range<C> {}
impl<C: Component> FusedIterator for Range<C> {}
// TODO: TrustedLen

pub type RowRange = Range<Row>;
pub type ColumnRange = Range<Column>;

// TODO: Error implementation

/// Error indicating that a Row or Column was out of bounds.
///
///  Note that the bounds expressed in this error are half inclusive; that is,
///  the lower bound in TooLow is an inclusive lower bound, but the upper bound
///  in TooHigh is an exclusive upper bound. This is consistent with the
///  conventional range representation of `low..high`
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

pub type RowRangeError = RangeError<Row>;
pub type ColumnRangeError = RangeError<Column>;
