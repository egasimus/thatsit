use crate::{*, widgets::*};

/// Trait for things that can go in a closure-built collection.
/// These collections are used for building render-lists in place.
pub trait Collectible {
    /// Used when building render trees.
    /// Thanks @steffahn for suggesting this!
    fn collect <'a> (self, collect: &mut Collector<'a, Self>) where Self: 'a + Sized {
        collect.0.push(Collected::Box(Box::new(self)));
    }
}

impl<'a, T, U> Collectible for dyn Output<T, U> + 'a {}

/// Wrapper that allows owned and borrowed items to be treated similarly.
pub enum Collected<'a, T> {
    Box(Box<T>),
    Ref(&'a T),
    None
}

/// Callable struct that collects Collecteds into itself.
pub struct Collector<'a, T>(pub Vec<Collected<'a, T>>);

impl<'a, T: Collectible> Collector<'a, T> {
    pub fn collect (collect: impl Fn(&mut Collector<'a, T>)) -> Self {
        let mut items = Self(vec![]);
        collect(&mut items);
        items
    }
}

impl<'a, T: Collectible> std::fmt::Debug for Collected<'a, T> {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Collected({})", match self {
            Self::Box(_) => "Box",
            Self::Ref(_) => "Ref",
            Self::None   => ".x.",
        })
    }
}

impl<'a, T: Output<U, V>, U, V> Output<U, V> for Collected<'a, T> {
    fn render (&self, context: &mut U) -> Result<Option<V>> {
        Ok(match self {
            Self::Box(item) => (*item).render(context)?,
            Self::Ref(item) => (*item).render(context)?,
            Self::None => None
        })
    }
}

impl<'a, T: Collectible> FnOnce<(T, )> for Collector<'a, T> {
    type Output = ();
    extern "rust-call" fn call_once (mut self, args: (T,)) -> Self::Output {
        self.call_mut(args)
    }
}

impl<'a, T: Collectible> FnMut<(T, )> for Collector<'a, T> {
    extern "rust-call" fn call_mut (&mut self, args: (T,)) -> Self::Output {
        args.0.collect(self)
    }
}

#[cfg(test)]
mod test {
    #[test] fn should_collect () {

    }
}
