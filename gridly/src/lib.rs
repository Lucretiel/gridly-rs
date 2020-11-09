#![no_std]

pub mod direction;
pub mod grid;
pub mod location;
pub mod range;
pub mod rotation;
pub mod vector;

/// The shorthand module includes quick single-character shorthand constructors
/// for the common gridly types ([`Row`][location::Row],
/// [`Column`][location::Column], [`Vector`][vector::Vector], and
/// [`Location`][location::Location]).
///
/// These are not included in the [`prelude`][prelude] because we don't want a
/// bulk import to unexpectedly add a bunch of single-character names to the
/// namespace.
pub mod shorthand {
    use crate::location::{Column, Location, Row};
    use crate::vector::{Columns, Rows, Vector};

    // TODO: make these shorthands smart enough to be either `Row` or `Rows` using
    // generics. Perhaps they can implement both kinds of component?

    /// Shorthand to create a [`Row`].
    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn R(row: impl Into<Row>) -> Row {
        row.into()
    }

    /// Shorthand to create a [`Column`].
    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn C(column: impl Into<Column>) -> Column {
        column.into()
    }

    /// Shorthand to create a [`Vector`].
    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn V(rows: impl Into<Rows>, columns: impl Into<Columns>) -> Vector {
        Vector::new(rows, columns)
    }

    /// Shorthand to create a [`Location`].
    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn L(row: impl Into<Row>, column: impl Into<Column>) -> Location {
        Location::new(row, column)
    }
}

/// The gridly prelude includes all traits and common structs.
///
/// Note that the prelude does not include the single-character [shorthand]
/// functions, so as not to pollute your namespace with single-character
/// identifiers on a bulk import.
pub mod prelude {
    #[doc(inline)]
    pub use crate::direction::{Direction, Down, Left, Right, Up, EACH_DIRECTION};

    #[doc(inline)]
    pub use crate::grid::{BoundsError, Grid, GridBounds, GridMut, GridSetter};

    #[doc(inline)]
    pub use crate::location::{
        Column, Component as LocationComponent, Location, LocationLike, Row,
    };

    #[doc(inline)]
    pub use crate::range::{ColumnRange, ColumnRangeError, LocationRange, RowRange, RowRangeError};

    #[doc(inline)]
    pub use crate::vector::{
        Columns, Component as VectorComponent, Rows, Vector, VectorLike, DIAGONAL_ADJACENCIES,
        ORTHOGONAL_ADJACENCIES, TOUCHING_ADJACENCIES,
    };

    #[doc(inline)]
    pub use crate::rotation::{Anticlockwise, Clockwise, Rotation};
}
