use std::ops::{Add, AddAssign, Sub, SubAssign};

use derive_more::*;

use crate::vector::{Component as VecComponent, Rows, Columns};

/// A component of a [`Location`], either a [`Row`] or a [`Columns`]
pub trait Component: Sized + From<isize> + Into<isize> {
    /// The converse component ([`Row`] to [`Column`], or vice versa)
    type Converse: Component;

    /// The associated vector component
    type Distance: VecComponent;

    /// Get this component type from a [`Location`]
    fn from_location(location: &Location) -> Self;

    /// Combine this component with its converse to create a [`Location`]
    fn combine(self, other: Self::Converse) -> Location;
}

macro_rules! make_component {
    (
        $Name:ident,
        $Converse:ident,
        $Distance:ident,
        $from_loc:ident,
        ($self:ident, $other:ident) =>
        ($first:ident, $second:ident)
    ) => {
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, From, Into)]
        #[repr(transparent)]
        pub struct $Name(pub isize);

        impl Add<$Converse> for $Name {
            type Output = Location;

            fn add(self, rhs: $Converse) -> Location {
                self.combine(rhs)
            }
        }

        impl Add<$Distance> for $Name {
            type Output = $Name;

            fn add(self, rhs: $Distance) -> $Name {
                $Name(self.0 + rhs.0)
            }
        }

        impl AddAssign<$Distance> for $Name {
            fn add_assign(&mut self, rhs: $Distance) {
                self.0 += rhs.0
            }
        }

        impl Sub<$Distance> for $Name {
            type Output = $Name;

            fn sub(self, rhs: $Distance) -> $Name {
                $Name(self.0 - rhs.0)
            }
        }

        impl SubAssign<$Distance> for $Name {
            fn sub_assign(&mut self, rhs: $Distance) {
                self.0 -= rhs.0
            }
        }


        impl Component for $Name {
            type Converse = $Converse;
            type Distance = $Distance;

            fn from_location(loc: &Location) -> Self {
                loc.$from_loc
            }

            fn combine($self, $other: Self::Converse) -> Location {
                Location::new($first, $second)
            }
        }
    }
}

make_component!{Row, Column, Rows, row, (self, other) => (self, other)}
make_component!{Column, Row, Columns, column, (self, other) => (other, self)}

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct Location {
    pub row: Row,
    pub column: Column,
}

impl Location {
    pub fn new(row: impl Into<Row>, column: impl Into<Column>) -> Self {
        Location {
            row: row.into(),
            column: column.into(),
        }
    }

    pub fn origin() -> Self {
        Location::new(0, 0)
    }

    pub fn get_component<T: Component>(&self) -> T {
        T::from_location(self)
    }
}
