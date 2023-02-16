#![feature(fn_traits, unboxed_closures)]

pub mod engines;
pub mod layouts;
pub mod widgets;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Engine<T> {
    fn run (self, context: T) -> Result<Self> where Self: Sized;
    fn done (&self) -> bool;
}

pub trait Input<T, U> {
    fn handle (&mut self, context: T) -> Result<Option<U>>;
}

pub trait Proxy<T> {
    fn get (&self) -> &T;
    fn get_mut (&mut self) -> &mut T;
}

impl<T, U, V: Input<T, U>> Input<T, U> for dyn Proxy<V> {
    fn handle (&mut self, context: T) -> Result<Option<U>> {
        self.get_mut().handle(context)
    }
}

pub trait Output<T, U> {
    fn render (&self, context: &mut T) -> Result<Option<U>>;

    /// Add this output to a `Collector`. Used when building render trees in-place.
    /// Thanks @steffahn for suggesting this!
    fn collect <'a> (self, collect: &mut Collector<'a, T, U>) where Self: 'a + Sized {
        collect.0.push(Collected::Box(Box::new(self)));
    }
}

/// Widgets work the same when passed as immutable references.
impl<T, U, V: Output<T, U>> Output<T, U> for &V {
    fn render (&self, context: &mut T) -> Result<Option<U>> {
        (*self).render(context)
    }
    fn collect <'a> (self, collect: &mut Collector<'a, T, U>) where Self: 'a + Sized {
        collect.0.push(Collected::Ref(self));
    }
}

/// Widgets work the same when boxed.
impl<'a, T, U> Output<T, U> for Box<dyn Output<T, U> + 'a> {
    fn render (&self, context: &mut T) -> Result<Option<U>> {
        (**self).render(context)
    }
    fn collect <'b> (self, collect: &mut Collector<'b, T, U>) where Self: 'b + Sized {
        collect.0.push(Collected::Box(self));
    }
}

/// Wrapper that allows owned and borrowed items to be treated similarly.
pub enum Collected<'a, T, U> {
    Box(Box<dyn Output<T, U> + 'a>),
    Ref(&'a dyn Output<T, U>),
    None
}

/// Callable struct that collects Collecteds into itself.
pub struct Collector<'a, T, U>(pub Vec<Collected<'a, T, U>>);

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

impl<'a, T, U, V: Output<T, U> + 'a> FnOnce<(V, )> for Collector<'a, T, U> {
    type Output = ();
    extern "rust-call" fn call_once (mut self, args: (V,)) -> Self::Output {
        self.call_mut(args)
    }
}

impl<'a, T, U, V: Output<T, U> + 'a> FnMut<(V, )> for Collector<'a, T, U> {
    extern "rust-call" fn call_mut (&mut self, args: (V,)) -> Self::Output {
        args.0.collect(self)
    }
}

impl<'a, T, U> Collector<'a, T, U> {
    pub fn collect (collect: impl Fn(&mut Collector<'a, T, U>)) -> Self {
        let mut items = Self(vec![]);
        collect(&mut items);
        items
    }
}


#[cfg(test)]
mod test {
    use crate::*;

    impl Output<(), ()> for String {
        fn render (&self, context: &mut ()) -> Result<Option<()>> {
            Ok(Some(()))
        }
    }
    impl Output<(), ()> for &str {
        fn render (&self, context: &mut ()) -> Result<Option<()>> {
            Ok(Some(()))
        }
    }

    struct NullWidget;

    impl Input<(), ()> for NullWidget {
        fn handle (&mut self, context: ()) -> Result<Option<()>> {
            Ok(Some(()))
        }
    }
    impl Output<(), ()> for NullWidget {
        fn render (&self, context: &mut ()) -> Result<Option<()>> {
            Ok(Some(()))
        }
    }

    impl<'a, X: Input<(), ()> + Output<(), ()>> Engine<&mut ()> for X {
        fn done (&self) -> bool {
            true
        }
        fn run (self, context: &mut ()) -> Result<Self> {
            Ok(self)
        }
    }

    #[test]
    fn should_run () -> Result<()> {
        let mut app = NullWidget;
        app.handle(())?;
        app.render(&mut ())?;
        app.run(&mut ())?;
        Ok(())
    }

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
