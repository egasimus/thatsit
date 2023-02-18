//! Render widgets next to each other

use crate::*;
use super::*;

pub struct Columns<'a, T, U>(pub(crate) Vec<Collected<'a, T, U>>);

impl<'a, T, U> Columns<'a, T, U> {
    pub fn new () -> Self {
        Self(vec![])
    }
}

impl<'a, T, U, V: Collectible<'a, T, U>> Collection<'a, T, U, V> for Columns<'a, T, U> {
    /// Add a column to this collection
    fn add (&mut self, widget: V) -> &mut Self {
        self.0.push(widget.collected());
        self
    }
}
