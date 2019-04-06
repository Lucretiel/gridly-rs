//#![no_std]

pub mod direction;
pub mod grid;
pub mod location;
pub mod vector;

pub use direction::{Direction, Down, Left, Right, Up};
pub use grid::{Grid, GridBounds, GridBoundsExt, GridExt};
pub use location::{Column, Location, Row};
pub use vector::{Columns, Rows, Vector};

use core::marker::PhantomData;
use core::iter;
use crate::location::Component as LocComponent;
use crate::vector::Component as VecComponent;

struct BasicGrid<T, Major: location::Component> {
    dimensions: Vector,
    storage: Vec<T>,
    phantom: PhantomData<Major>,
}

impl<T, Major: location::Component> BasicGrid<T, Major> {
    fn new(dimensions: Vector, init: T) -> Option<Self> where T: Clone {
        if dimensions.rows < 0 {
            None
        } else if dimensions.columns < 0 {
            None
        } else {
            dimensions.rows.0.checked_mul(dimensions.columns.0).map(|volume| {
                BasicGrid {
                    dimensions,
                    storage: iter::repeat(init).take(volume as usize).collect(),
                    phantom: PhantomData,
                }
            })
        }
    }

    unsafe fn index_for_location(&self, loc: &Location) -> usize {
        // For ease of understanding, the variable names in this function assume
        // a row-major ordering. Swap them in your head if Major = Column.

        let column_index: Major::Converse = loc.get_component();
        let row_index: Major = loc.get_component();
        let row_size: Major::Distance = self.dimensions.get_component();

        ((column_index.value() * row_size.value()) + row_index.value()) as usize
    }
}

impl<T, Major: location::Component> GridBounds for BasicGrid<T, Major> {}
