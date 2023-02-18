//! Render widgets below each other

use crate::*;

pub struct Rows<'a, T, U>(pub(crate) Vec<Collected<'a, T, U>>);

impl<'a, T, U> Rows<'a, T, U> {
    pub fn new () -> Self {
        Self(vec![])
    }
}

impl<'a, T, U, V: Collectible<'a, T, U>> Collection<'a, T, U, V> for Rows<'a, T, U> {
    /// Add a row to this collection
    fn add (&mut self, widget: V) -> &mut Self {
        self.0.push(widget.collected());
        self
    }
}
