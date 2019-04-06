use core::iter::FusedIterator;

use crate::location::component;
use crate::location::{Component, Location};
use crate::vector::Vector;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Range<C: Component> {
    index: C,
    range: component::Range<C::Converse>,
}

impl<C: Component> Range<C> {
    pub fn new(index: C, range: component::Range<C::Converse>) -> Self {
        Range { index, range }
    }

    pub fn bounded(index: C, start: C::Converse, end: C::Converse) -> Self {
        Self::new(index, component::Range::bounded(start, end))
    }

    pub fn span(index: C, start: C::Converse, size: <C::Converse as Component>::Distance) -> Self {
        Self::new(index, component::Range::span(start, size))
    }

    pub fn component_range(&self) -> component::Range<C::Converse> {
        self.range.clone()
    }

    pub fn index(&self) -> C {
        self.index
    }

    pub fn start(&self) -> Location {
        self.range.start().combine(self.index)
    }

    pub fn end(&self) -> Location {
        self.range.end().combine(self.index)
    }

    pub fn size(&self) -> Vector {
        self.range.size().into()
    }
}

impl<C: Component> Iterator for Range<C> {
    type Item = Location;

    #[inline]
    fn next(&mut self) -> Option<Location> {
        self.range.next().map(move |cross| cross.combine(self.index))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Location> {
        self.range.nth(n).map(move |cross| cross.combine(self.index))
    }

    #[inline]
    fn last(self) -> Option<Location> {
        self.range.last().map(move |cross| cross.combine(self.index))
    }
}

impl<C: Component> DoubleEndedIterator for Range<C> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range.next_back().map(move |cross| cross.combine(self.index))
    }
}

impl<C: Component> FusedIterator for Range<C> {}
impl<C: Component> ExactSizeIterator for Range<C> {}
// TODO: TrustedLen
