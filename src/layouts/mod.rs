//! Abstract layout primitives.

mod align; pub use align::*;
mod columns; pub use columns::*;
mod fixed; pub use fixed::*;
mod focus; pub use focus::*;
mod layers; pub use layers::*;
mod max; pub use max::*;
mod min; pub use min::*;
mod offset; pub use offset::*;
mod rows; pub use rows::*;
mod text; pub use text::*;

use std::{fmt::{Debug}};

/// An axis in space
#[derive(Copy, Clone, Debug)]
pub enum Axis {
    /// Time
    T = 0,
    X,
    Y,
    Z,
}

/// A unit of distance.
pub trait Unit: Copy {
    const NIL: Self;
}

impl Unit for f32 {
    const NIL: Self = 0f32;
}

impl Unit for i32 {
    const NIL: Self = 0i32;
}

impl Unit for u16 {
    const NIL: Self = 0u16;
}

impl Unit for u32 {
    const NIL: Self = 0u32;
}

/// A point in an N-dimensional space.
pub trait Point<const N: usize, D: Unit> {
    fn nth (&self, n: usize) -> D;
}

impl<U: Unit> Point<2, U> for [U; 2] {
    fn nth (&self, n: usize) -> U { self[n] }
}

impl<U: Unit> Point<2, U> for (U, U) {
    fn nth (&self, n: usize) -> U {
        match n { 0 => self.0, 1 => self.1, _ => panic!() }
    }
}

impl<U: Unit> dyn Point<2, U> {
    pub fn x (&self) -> U {
        self.nth(0)
    }
    pub fn y (&self) -> U {
        self.nth(1)
    }
}

/// A rectangle, defined by position and size.
///
/// TODO - make signed, counting position (0, 0) from center of FOV;
///      - add method for converting to unsigned (corner at 0,0)
pub trait Rect<U: Unit> {
    fn x (&self) -> U;
    fn y (&self) -> U;
    fn w (&self) -> U;
    fn h (&self) -> U;
    fn size (&self) -> [U;2] {
        [self.w(), self.h()]
    }
}

impl<U: Unit> Rect<U> for (U, U, U, U) {
    fn x (&self) -> U {
        self.0
    }
    fn y (&self) -> U {
        self.1
    }
    fn w (&self) -> U {
        self.2
    }
    fn h (&self) -> U {
        self.3
    }
}

impl<U: Unit> Rect<U> for [U; 4] {
    fn x (&self) -> U {
        self[0]
    }
    fn y (&self) -> U {
        self[1]
    }
    fn w (&self) -> U {
        self[2]
    }
    fn h (&self) -> U {
        self[3]
    }
}

#[cfg(test)]
mod test {

    use crate::{*, layouts::*};
    use std::io::Write;

    #[test]
    fn should_stack_builder () -> Result<()> {

        struct Widget;

        impl Output<(), ()> for Widget {
            fn render (&self, engine: &mut ()) -> Result<Option<()>> {
                Columns::new()
                    .add(&"String")
                    .add(String::from("String"))
                    .add(Rows::new()
                        .add(&"String")
                        .add(String::from("String"))
                        .add(Layers::new()
                            .add(&"String")
                            .add(String::from("String"))))
                    .render(engine)
            }
        }

        Output::<(), ()>::render(&Widget, &mut ())?;

        Ok(())
    }

}
