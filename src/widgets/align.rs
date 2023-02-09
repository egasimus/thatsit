use crate::*;

/// Direction in which to perform alignment
#[derive(Copy, Clone, Default, Debug)]
pub enum Align {
    TopLeft,
    Top,
    TopRight,
    Left,
    #[default] Center,
    Right,
    BottomLeft,
    Bottom,
    BottomRight
}

/// Wraps a widget, applying alignment to it.
#[derive(Copy, Clone, Default)]
pub struct Aligned<T>(
    /// The line along which this component is aligned
    pub Align,
    /// The component to the contents of which the alignment is applied
    pub T
);

impl<T, U, V: Output<T, U>> Output<T, U> for Aligned<V> {
    fn render (&self, context: &mut T) -> Result<Option<U>> {
        self.1.render(context)
    }
}

