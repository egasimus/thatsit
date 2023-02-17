//! Constrain a widget to a fixed size

use crate::*;
use super::*;

#[derive(Debug)]
/// Set exact size
pub enum Fixed<Unit, W> {
    /// The contained widget will have a fixed horizontal size
    X(Unit, W),
    /// The contained widget will have a fixed vertical size
    Y(Unit, W),
    /// The contained widget will have a fixed size along both axes
    XY((Unit, Unit), W),
}

impl<Unit, T> Proxy<T> for Fixed<Unit, T> {
    fn get (&self) -> &T {
        match self { Self::X(_, w)  => w, Self::Y(_, w)  => w, Self::XY(_, w) => w }
    }
    fn get_mut (&mut self) -> &mut T {
        match self { Self::X(_, w)  => w, Self::Y(_, w)  => w, Self::XY(_, w) => w }
    }
}
