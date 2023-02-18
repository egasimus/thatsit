use crate::*;

/// A collection of widgets
pub trait Collection<'a, T, U> {
    fn add (&mut self, widget: Collected<'a, T, U>) -> &mut Self;
}

/// Callable struct that collects Collecteds from a closure into itself.
pub struct Collector<'a, T, U>(pub Vec<Collected<'a, T, U>>);

impl<'a, T, U> Collector<'a, T, U> {
    /// Call this collector's closure, collecting the items
    pub fn collect_items (collect: impl Fn(&mut Collector<'a, T, U>)) -> Self {
        let mut items = Self(vec![]);
        collect(&mut items);
        items
    }
}

impl<'a, T, U> Collection<'a, T, U> for Collector<'a, T, U> {
    fn add (&mut self, widget: Collected<'a, T, U>) -> &mut Self {
        self.0.push(widget);
        self
    }
}

impl<'a, T, U, V: Collectible<'a, T, U>> FnOnce<(V, )> for Collector<'a, T, U> {
    type Output = ();
    extern "rust-call" fn call_once (mut self, args: (V,)) -> Self::Output {
        self.call_mut(args)
    }
}

impl<'a, T, U, V: Collectible<'a, T, U>> FnMut<(V, )> for Collector<'a, T, U> {
    extern "rust-call" fn call_mut (&mut self, (collectible,): (V,)) -> Self::Output {
        collectible.collect_into(self)
    }
}

pub trait Collectible<'a, T, U> {
    /// Add this output to a `Collector`. Used when building render trees in-place.
    /// Thanks @steffahn for suggesting this!
    fn collect_into (self, collector: &mut Collector<'a, T, U>) where Self: Sized;
}

impl<'a, T, U, V: Output<T, U>> Collectible<'a, T, U> for &'a V {
    fn collect_into (self, collector: &mut Collector<'a, T, U>) where Self: Sized {
        collector.add(Collected::Ref(self));
    }
}

impl<'a, T, U> Collectible<'a, T, U> for Box<dyn Output<T, U> + 'a> {
    fn collect_into (self, collector: &mut Collector<'a, T, U>) where Self: Sized {
        collector.add(Collected::Box(Box::new(self)));
    }
}

/// Wrapper that allows owned and borrowed items to be treated similarly.
pub enum Collected<'a, T, U> {
    Box(Box<dyn Output<T, U> + 'a>),
    Ref(&'a dyn Output<T, U>),
    None
}

impl<'a, T, U> std::fmt::Debug for Collected<'a, T, U> {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Collected({})", match self {
            Self::Box(_) => "Box",
            Self::Ref(_) => "Ref",
            Self::None   => ".x.",
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

//pub trait Collection<'a, T, U> {
    //fn add (self, widget: impl Output<T, U>) -> Self where Self: Sized {
        //widget.collect(self);
        //self
    //}
//}

#[cfg(test)]
mod test {
    use crate::{*, engines::null::*};

    #[test]
    fn should_collect () {
        let widget = NullWidget;
        let items = Collector::collect(|add|{
            add("String");
            add(String::from("String"));
            add(NullWidget);
            add(&widget);
        });
    }
}
