//! Abstract layout primitives.

mod align;
pub use align::*;

mod fixed;
pub use fixed::*;

mod focus;
pub use focus::*;

mod max;
pub use max::*;

mod min;
pub use min::*;

mod offset;
pub use offset::*;

mod stacked;
pub use stacked::*;

mod text;
pub use text::*;

use std::{fmt::{Display, Debug}};

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

//#[derive(Copy, Clone, Debug)]
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

/// A box, consisting of two vectors - position and size.
/// When N=2, this represents a rectangle and so on.
/// TODO replace Area with this?
/// TODO - make signed;
///      - count position (0, 0) from center of FOV;
///      - add method for converting to unsigned (corner at 0,0)
pub trait Rect<U: Unit> {
    fn x (&self) -> U;
    fn y (&self) -> U;
    fn w (&self) -> U;
    fn h (&self) -> U;
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

#[derive(Copy, Clone, Debug)]
pub enum Axis {
    X = 0,
    Y,
    Z,
}

/// A rectangle on the screen in (X, Y, W, H) format, from top left.
#[derive(Copy, Clone, Default, Debug)]
pub struct Area<U: Unit>(
    pub U,
    pub U,
    pub U,
    pub U
);

impl<U: Unit + Ord + Display + Debug> Area<U> {
}

impl<U: Unit> Area<U> {
    ///// Move the cursor to the upper left corner of the area
    //pub fn home <'a> (&'a self, out: &'a mut dyn Write) -> Result<&'a mut dyn Write> {
        //out.queue(MoveTo(self.x(), self.y()))
    //}
    #[inline]
    pub fn x (&self) -> U {
        self.0
    }
    #[inline]
    pub fn y (&self) -> U {
        self.1
    }
    #[inline]
    pub fn w (&self) -> U {
        self.2
    }
    #[inline]
    pub fn h (&self) -> U {
        self.3
    }
    #[inline]
    pub fn size (&self) -> (U, U) {
        (self.w(), self.h())
    }
}
