//! Render widgets on top of each other

use crate::*;
use super::*;

pub struct Layers<'a, T, U>(pub(crate) Vec<Collected<'a, T, U>>);

impl<'a, T, U> Layers<'a, T, U> {
    pub fn new () -> Self {
        Self(vec![])
    }
}

impl<'a, T, U> Collection<'a, T, U> for Layers<'a, T, U> {
    /// Add a layer to this collection
    fn add (mut self, widget: impl Output<T, U> + 'a) -> Self {
        self.0.push(widget.into_collected());
        self
    }
}
