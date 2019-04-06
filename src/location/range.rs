use crate::location::component;
use crate::location::{Component, Location};
use crate::vector::Vector;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Range<C: Component> {
    cross: C::Converse,
    range: component::Range<C>,
}

impl<C: Component> Range<C> {
    pub fn new(cross: C::Converse, range: component::Range<C>) -> Self {
        Range { cross, range }
    }

    pub fn bounded(cross: C::Converse, start: C, end: C) -> Self {
        Self::new(cross, component::Range::bounded(start, end))
    }

    pub fn span(cross: C::Converse, start: C, size: C::Distance) -> Self {
        Self::new(cross, component::Range::span(start, size))
    }

    // We return an object, rather than a reference, on the assumption that
    // ranges are cheap to copy and that the interior range is an implementation
    // detail that may go away
    pub fn component_range(&self) -> component::Range<C> {
        self.range.clone()
    }

    pub fn cross(&self) -> C::Converse {
        self.cross
    }

    pub fn start(&self) -> Location {
        self.range.start().combine(self.cross)
    }

    pub fn end(&self) -> Location {
        self.range.end().combine(self.cross)
    }

    pub fn size(&self) -> Vector {
        self.range.size().into()
    }
}

impl<C: Component> Iterator for Range<C> {
    type Item = Location;

    #[inline]
    fn next(&mut self) -> Option<Location> {
        self.range.next().map(move |idx| idx.combine(self.cross))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Location> {
        self.range.nth(n).map(move |idx| idx.combine(self.cross))
    }

    #[inline]
    fn last(self) -> Option<Location> {
        self.range.last().map(move |idx| idx.combine(self.cross))
    }
}
