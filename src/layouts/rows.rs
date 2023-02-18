use crate::*;

/// Renders widgets below each other.
pub struct Rows<'a, T, U>(pub(crate) Vec<Collected<'a, T, U>>);

impl<'a, T, U> Rows<'a, T, U> {
    pub fn new () -> Self {
        Self(vec![])
    }
}

impl<'a, T, U> Collection<'a, T, U> for Rows<'a, T, U> {
    /// Add a row to this collection
    fn add (mut self, widget: impl Output<T, U> + 'a) -> Self {
        self.0.push(widget.into_collected());
        self
    }
}
