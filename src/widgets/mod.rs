pub mod align;
pub mod focus;

pub mod text;
pub mod offset;
pub mod fixed;
pub mod min;
pub mod max;
pub mod stacked;

use std::{io::{Error, ErrorKind}, fmt::{Display, Debug}};

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

#[derive(Copy, Clone, Debug)]
pub struct Point<const N: usize, D: Unit>(
    pub [D; N],
);

impl<const N: usize, D: Unit> Default for Point<N, D> {
    fn default () -> Self {
        Self([D::NIL; N])
    }
}

/// A box, consisting of two vectors - position and size.
/// When N=2, this represents a rectangle and so on.
/// TODO replace Area with this?
#[derive(Copy, Clone, Default, Debug)]
pub struct Rect<const N: usize, D: Unit>(
    /// Position
    pub Point<N, D>,
    /// Size
    pub Point<N, D>
);

#[derive(Copy, Clone, Debug)]
pub enum Axis {
    X = 1,
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
    /// Return an error if this area is larger than the minimum needed size
    pub fn expect_min (&self, (min_w, min_h): (U, U)) -> std::io::Result<&Self> {
        if self.w() < min_w || self.h() < min_h {
            let msg = format!("no space ({:?} < {}x{})", self, min_w, min_h);
            Err(Error::new(ErrorKind::Other, msg))
        } else {
            Ok(self)
        }
    }
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
