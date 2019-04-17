pub mod direction;
pub mod grid;
pub mod location;
pub mod vector;

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
