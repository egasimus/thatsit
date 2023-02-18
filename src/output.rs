
use crate::*;

use std::fmt::{Debug, Formatter};

/// Displays information to the user in the format specified by the engine.
pub trait Output<T, U> {
    /// Render this component in an engine-specific way
    fn render (&self, engine: &mut T) -> Result<Option<U>>;
    /// Wrap this output in the appropriate `Collected` variant.
    fn into_collected <'a> (self) -> Collected<'a, T, U> where Self: Sized + 'a {
        Collected::Box(Box::new(self))
    }
}

/// Rendering works across immutable references.
impl<T, U, V: Output<T, U>> Output<T, U> for &V {
    fn render (&self, engine: &mut T) -> Result<Option<U>> {
        (*self).render(engine)
    }
    /// References to items are added as `Collected::Ref`.
    fn into_collected <'a> (self) -> Collected<'a, T, U> where Self: Sized + 'a {
        Collected::Ref(self)
    }
}

/// Rendering works across mutable references.
impl<T, U, V: Output<T, U>> Output<T, U> for &mut V {
    fn render (&self, engine: &mut T) -> Result<Option<U>> {
        (**self).render(engine)
    }
    /// Mutable references to items are added as `Collected::Ref`.
    fn into_collected <'a> (self) -> Collected<'a, T, U> where Self: Sized + 'a {
        Collected::Ref(self)
    }
}

/// Rendering works across boxes.
impl<'a, T, U> Output<T, U> for Box<dyn Output<T, U> + 'a> {
    fn render (&self, engine: &mut T) -> Result<Option<U>> {
        (**self).render(engine)
    }
    /// Boxed items are added as `Collected::Box`.
    fn into_collected <'b> (self) -> Collected<'b, T, U> where Self: Sized + 'b {
        Collected::Box(self)
    }
}

/// Rendering a `None` renders nothing, implementing optional widgets.
/// Note that setting an optional widget to `None` clobbers its state.
impl<T, U, V: Output<T, U>> Output<T, U> for Option<V> {
    fn render (&self, engine: &mut T) -> Result<Option<U>> {
        match self {
            Some(widget) => widget.render(engine),
            None => Ok(None)
        }
    }
}

/// A collection of widgets.
pub trait Collection<'a, T, U> {
    fn add (self, widget: impl Output<T, U> + 'a) -> Self;
}

/// Wrapper that allows owned and borrowed items to be treated similarly.
/// Thanks @steffahn for suggesting the overall approach!
pub enum Collected<'a, T, U> {
    Box(Box<dyn Output<T, U> + 'a>),
    Ref(&'a (dyn Output<T, U> + 'a)),
    None
}

impl<'a, T, U> Debug for Collected<'a, T, U> {
    fn fmt (&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Collected({})", match self {
            Self::Box(_) => "Box",
            Self::Ref(_) => "Ref",
            Self::None   => "Nil.",
        })
    }
}

impl<'a, T, U> Output<T, U> for Collected<'a, T, U> {
    fn render (&self, engine: &mut T) -> Result<Option<U>> {
        Ok(match self {
            Self::Box(item) => (*item).render(engine)?,
            Self::Ref(item) => (*item).render(engine)?,
            Self::None => None
        })
    }
}

impl<'a, T, U> Collector<'a, T, U> {
    /// Pass this collector to a closure which adds items to it
    pub fn collect_items (collect: impl Fn(&mut Collector<'a, T, U>)) -> Self {
        let mut items = Self(vec![]);
        collect(&mut items);
        items
    }
    /// Add an item to this collector
    fn add (&mut self, widget: Collected<'a, T, U>) -> &mut Self {
        self.0.push(widget);
        self
    }
}

/// Callable struct that collects Collecteds from a closure into itself.
pub struct Collector<'a, T, U>(pub Vec<Collected<'a, T, U>>);

/// Calling the collector with an item adds it to the collection.
impl<'a, T, U, V: Output<T, U> + 'a> FnOnce<(V, )> for Collector<'a, T, U> {
    type Output = ();
    extern "rust-call" fn call_once (mut self, args: (V,)) -> Self::Output {
        self.call_mut(args)
    }
}

/// Calling the collector with an item adds it to the collection.
impl<'a, T, U, V: Output<T, U> + 'a> FnMut<(V, )> for Collector<'a, T, U> {
    extern "rust-call" fn call_mut (&mut self, (widget,): (V,)) -> Self::Output {
        self.add(widget.into_collected());
    }
}

#[cfg(test)]
mod test {

    use crate::{*, engines::null::*};

    #[test]
    fn should_collect_callback () -> Result<()> {

        Collector::<(), ()>::collect_items(|add|{
            add(&"String");
            add(String::from("String"));
            add(NullWidget);
            add(&NullWidget);
        });

        Ok(())

    }

    #[test]
    fn should_collect_builder () -> Result<()> {

        Collector::<(), ()>::collect_items(|add|{
            add(&"String");
            add(String::from("String"));
            add(NullWidget);
            add(&NullWidget);
        });

        Ok(())

    }

}
