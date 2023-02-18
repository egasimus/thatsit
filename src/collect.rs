use crate::*;

use std::fmt::{Debug, Formatter};

/// A collection of widgets
pub trait Collection<'a, T, U> {
    fn add (&mut self, widget: Collected<'a, T, U>) -> &mut Self;
}

/// Callable struct that collects Collecteds from a closure into itself.
pub struct Collector<'a, T, U>(pub Vec<Collected<'a, T, U>>);

/// An item that can be added into a collection.
pub trait Collectible<'a, T, U> {
    /// Add this output to a `Collector`. Used when building render trees in-place.
    /// Thanks @steffahn for suggesting this!
    fn collect_into (self, collector: &mut Collector<'a, T, U>) where Self: Sized;
}

/// Wrapper that allows owned and borrowed items to be treated similarly.
pub enum Collected<'a, T, U> {
    Box(Box<dyn Output<T, U> + 'a>),
    Ref(&'a dyn Output<T, U>),
    None
}

impl<'a, T, U> Collector<'a, T, U> {
    /// Pass this collector to a closure which adds items to it
    pub fn collect_items (collect: impl Fn(&mut Collector<'a, T, U>)) -> Self {
        let mut items = Self(vec![]);
        collect(&mut items);
        items
    }
}

/// A Collector is only one kind of Collection:
/// the kind where you add items by calling the Collector
impl<'a, T, U> Collection<'a, T, U> for Collector<'a, T, U> {
    /// Add an item to this collector
    fn add (&mut self, widget: Collected<'a, T, U>) -> &mut Self {
        self.0.push(widget);
        self
    }
}

/// Calling the collector with an item adds it to the collection.
impl<'a, T, U, V: Collectible<'a, T, U>> FnOnce<(V, )> for Collector<'a, T, U> {
    type Output = ();
    extern "rust-call" fn call_once (mut self, args: (V,)) -> Self::Output {
        self.call_mut(args)
    }
}

/// Calling the collector with an item adds it to the collection.
impl<'a, T, U, V: Collectible<'a, T, U>> FnMut<(V, )> for Collector<'a, T, U> {
    extern "rust-call" fn call_mut (&mut self, (collectible,): (V,)) -> Self::Output {
        collectible.collect_into(self)
    }
}

/// References to items are added as `Collected::Ref`.
impl<'a, T, U, V: Output<T, U>> Collectible<'a, T, U> for &'a V {
    fn collect_into (self, collector: &mut Collector<'a, T, U>) where Self: Sized {
        collector.add(Collected::Ref(self));
    }
}

/// Boxed items are added as `Collected::Box`.
impl<'a, T, U> Collectible<'a, T, U> for dyn Output<T, U> + 'a {
    fn collect_into (self, collector: &mut Collector<'a, T, U>) where Self: Sized {
        collector.add(Collected::Box(Box::new(self)));
    }
}

/// Boxed items are added as `Collected::Box`.
impl<'a, T, U> Collectible<'a, T, U> for Box<dyn Output<T, U> + 'a> {
    fn collect_into (self, collector: &mut Collector<'a, T, U>) where Self: Sized {
        collector.add(Collected::Box(self));
    }
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
    fn render (&self, context: &mut T) -> Result<Option<U>> {
        Ok(match self {
            Self::Box(item) => (*item).render(context)?,
            Self::Ref(item) => (*item).render(context)?,
            Self::None => None
        })
    }
}

#[cfg(test)]
mod test {

    use crate::{*, engines::null::*};

    #[test]
    fn should_collect_callback () -> Result<()> {

        Collector::<(), ()>::collect_items(|add|{
            add("String");
            add(String::from("String"));
            add(NullWidget);
            add(&NullWidgett);
        }).render(())

    }

    #[test]
    fn should_collect_builder () -> Result<()> {

        Collector::<(), ()>::collect_items(|add|{
            add("String");
            add(String::from("String"));
            add(NullWidget);
            add(&NullWidgett);
        }).render(())

    }

}
