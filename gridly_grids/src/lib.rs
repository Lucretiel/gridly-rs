//! A collection of basic 2D grids for use with the [gridly] grid library.
//! This crate is intended to fill most basic use cases that require 2D grids,
//! as well as serve as a sample implementation for how to implement gridly
//! grids.

mod sparse_grid;
mod vec_grid;

pub use sparse_grid::SparseGrid;
pub use vec_grid::VecGrid;
