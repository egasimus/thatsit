use crate::*;

/// Wrapper that allows owned and borrowed Widgets to be treated equally.
pub enum Layout<'a, T> {
    Box(Box<T>),
    Ref(&'a T),
    None
}

impl<'a, T> std::fmt::Debug for Layout<'a, T> {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Layout({})", match self {
            Self::Box(_) => "Box",
            Self::Ref(_) => "Ref",
            Self::None   => ".x.",
        })
    }
}

impl<'a, T: Output<U, V>, U, V> Output<U, V> for Layout<'a, T> {
    fn render (&self, context: &mut U) -> Result<Option<V>> {
        Ok(match self {
            Self::Box(item) => (*item).render(context)?,
            Self::Ref(item) => (*item).render(context)?,
            Self::None => None
        })
    }
}
