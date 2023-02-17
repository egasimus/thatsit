//! Constrain the maximum size of a widget

use crate::*;
use super::*;

/// Set maximum size for the contained widget across one or both axes
#[derive(Debug)]
pub enum Max<U, T> {
    X(U, T),
    Y(U, T),
    XY((U, U), T)
}

impl<Unit, T> Proxy<T> for Max<Unit, T> {
    fn get (&self) -> &T {
        match self { Self::X(_, w)  => w, Self::Y(_, w)  => w, Self::XY(_, w) => w }
    }
    fn get_mut (&mut self) -> &mut T {
        match self { Self::X(_, w)  => w, Self::Y(_, w)  => w, Self::XY(_, w) => w }
    }
}
