//! Constrain the minimum size of a widget

use crate::*;
use super::*;

/// Set minimum size for the contained widget across one or both axes
#[derive(Debug)]
pub enum Min<U, T> {
    X(U, T),
    Y(U, T),
    XY((U, U), T)
}

impl<Unit, T> Proxy<T> for Min<Unit, T> {
    fn get (&self) -> &T {
        match self { Self::X(_, w)  => w, Self::Y(_, w)  => w, Self::XY(_, w) => w }
    }
    fn get_mut (&mut self) -> &mut T {
        match self { Self::X(_, w)  => w, Self::Y(_, w)  => w, Self::XY(_, w) => w }
    }
}
