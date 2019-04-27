pub mod direction;
pub mod grid;
pub mod location;
pub mod vector;

pub mod shorthand {
    pub use crate::location::{Row as R, Column as C};
    use crate::vector::{Rows, Columns, Vector};
    use crate::location::Location;

    /// Shorthand to create a [`Vector`]
    #[allow(non_snake_case)]
    pub fn V(rows: impl Into<Rows>, columns: impl Into<Columns>) -> Vector {
        Vector::new(rows, columns)
    }

    /// Shorthand to create a [`Location`]
    #[allow(non_snake_case)]
    pub fn L(row: impl Into<R>, column: impl Into<C>) -> Location {
        Location::new(row, column)
    }
}

pub mod prelude {
    pub use crate::direction::{Direction, Down, Left, Right, Up};
    pub use crate::grid::{
        BaseGrid, BaseGridBounds, BaseGridMut, BaseGridSetter, Grid, GridBounds, GridMut,
        GridSetter,
    };
    pub use crate::location::component::{ColumnRange, RowRange};
    pub use crate::location::{
        Column, ColumnOrderedLocation, Component as LocationComponent, Location, OrderedLocation,
        Range as LocationRange, Row, RowOrderedLocation,
    };
    pub use crate::vector::{Columns, Component as VectorComponent, Rows, Vector};
}
