pub mod direction;
pub mod grid;
pub mod location;
pub mod range;
pub mod vector;

/// The shorthand module includes quick single-character shorthand constructors
/// for the common gridly types ([`Row`], [`Column`], [`Vector`], and [`Location`]).
///
/// These are not includes in the prelude because we don't want a bulk import to
/// unexpectedly add a bunch of single-character names to the namespace.
pub mod shorthand {
    use crate::location::{Column, Location, Row};
    use crate::vector::{Columns, Rows, Vector};

    // TODO: make these shorthands smart enough to be either `Row` or `Rows` using
    // generics. Perhaps they can implement both kinds of component?

    /// Shorthand to create a [`Row`]
    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn R(row: impl Into<Row>) -> Row {
        row.into()
    }

    /// Shorthand to create a [`Column`]
    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn C(column: impl Into<Column>) -> Column {
        column.into()
    }

    /// Shorthand to create a [`Vector`]
    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn V(rows: impl Into<Rows>, columns: impl Into<Columns>) -> Vector {
        Vector::new(rows, columns)
    }

    /// Shorthand to create a [`Location`]
    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn L(row: impl Into<Row>, column: impl Into<Column>) -> Location {
        Location::new(row, column)
    }
}

/// The gridly prelude includes all traits and common structs. It does not include
/// the single-letter [shorthand] constructors, though.
pub mod prelude {
    pub use crate::direction::{Direction, Down, Left, Right, Up, EACH_DIRECTION};
    pub use crate::grid::{
        BaseGrid, BaseGridBounds, BaseGridMut, BaseGridSetter, Grid, GridBounds, GridMut,
        GridSetter,
    };
    pub use crate::location::{
        Column, Component as LocationComponent, Location, LocationLike, Row,
    };
    pub use crate::range::{ColumnRange, LocationRange, RowRange};
    pub use crate::vector::{
        Columns, Component as VectorComponent, Rows, Vector, VectorLike, DIAGONAL_ADJACENCIES,
        ORTHOGONAL_ADJACENCIES, TOUCHING_ADJACENCIES,
    };
}
