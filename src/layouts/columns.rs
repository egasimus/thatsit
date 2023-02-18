use crate::*;

/// Renders widgets next to each other.
pub struct Columns<'a, T, U>(pub(crate) Vec<Collected<'a, T, U>>);

impl<'a, T, U> Columns<'a, T, U> {
    pub fn new () -> Self {
        Self(vec![])
    }
}

impl<'a, T, U> Collection<'a, T, U> for Columns<'a, T, U> {
    /// Add a column to this collection
    fn add (mut self, widget: impl Output<T, U> + 'a) -> Self {
        self.0.push(widget.into_collected());
        self
    }
}
