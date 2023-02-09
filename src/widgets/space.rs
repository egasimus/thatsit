#[derive(Debug)]
pub enum Axis {
    X = 0,
    Y,
    Z,
}

pub trait Vector<const D: usize> {}

/// A box, consisting of two vectors - position and size.
/// When N=2, this represents a rectangle and so on.
/// TODO replace Area with this?
#[derive(Copy, Clone, Default, Debug)]
pub struct Rect<const D: usize, V: Vector<D>> {
    pos:  V,
    size: V
}

/// A rectangle on the screen in (X, Y, W, H) format, from top left.
#[derive(Copy, Clone, Default, Debug)]
pub struct Area<U: Copy>(
    pub U,
    pub U,
    pub U,
    pub U
);

impl<Unit: Copy + Ord + std::fmt::Display + std::fmt::Debug> Area<Unit> {
    /// Return an error if this area is larger than the minimum needed size
    pub fn expect_min (&self, (min_w, min_h): (Unit, Unit)) -> std::io::Result<&Self> {
        if self.w() < min_w || self.h() < min_h {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other, format!("no space ({:?} < {}x{})", self, min_w, min_h)
            ))
        } else {
            Ok(self)
        }
    }
}

impl<U: Copy> Area<U> {
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
