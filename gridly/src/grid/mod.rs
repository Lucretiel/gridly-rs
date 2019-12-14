//! Core grid traits.
//!
//! This module contains the core grid traits that power gridly grids. These
//! traits (in combination with [`Vector`][crate::vector::Vector] and
//! [`Location`][crate::location::Location]) provide all of gridly's central reading,
//! writing, and bounds-checking functionality.

mod bounds;
mod setter;
mod view;
mod view_mut;

pub use bounds::{BoundsError, GridBounds};
pub use setter::GridSetter;
pub use view::{
    ColumnView, ColumnsView, DisplayAdapter, Grid, RowView, RowsView, SingleView, View,
};
pub use view_mut::GridMut;
