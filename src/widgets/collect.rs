use crate::{*, widgets::{*, layout::*}};

/// Trait for things that can go in a closure-built collection.
pub trait Collectible {
    /// Used when building render trees.
    /// Thanks @steffahn for suggesting this!
    fn collect <'a> (self, collect: &mut Collect<'a, Self>) where Self: 'a + Sized {
        collect.0.push(Layout::Box(Box::new(self)));
    }
}

/// These collections are used for building render-lists in place.
impl<'a, T, U> Collectible for dyn Output<T, U> + 'a {}

/// Callable struct that collects Layout-wrapped Widgets into itself.
pub struct Collect<'a, T: Collectible>(pub Vec<Layout<'a, T>>);

impl<'a, T: Collectible> Collect<'a, T> {
    pub fn collect (collect: impl Fn(&mut Collect<'a, T>)) -> Self {
        let mut items = Self(vec![]);
        collect(&mut items);
        items
    }
}

impl<'a, T: Collectible> FnOnce<(T, )> for Collect<'a, T> {
    type Output = ();
    extern "rust-call" fn call_once (mut self, args: (T,)) -> Self::Output {
        self.call_mut(args)
    }
}

impl<'a, T: Collectible> FnMut<(T, )> for Collect<'a, T> {
    extern "rust-call" fn call_mut (&mut self, args: (T,)) -> Self::Output {
        args.0.collect(self)
    }
}
