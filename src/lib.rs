#![feature(fn_traits, unboxed_closures)]

pub mod engines;
pub mod layouts;
pub mod widgets;

mod collect;
pub use collect::*;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Engine<T> {
    fn run (self, context: T) -> Result<T>;
}

pub trait Input<T, U> {
    fn handle (&mut self, engine: &mut T) -> Result<Option<U>>;
}

pub trait Proxy<T> {
    fn get (&self) -> &T;
    fn get_mut (&mut self) -> &mut T;
}

impl<T, U, V: Input<T, U>> Input<T, U> for dyn Proxy<V> {
    fn handle (&mut self, engine: &mut T) -> Result<Option<U>> {
        self.get_mut().handle(engine)
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

#[cfg(test)]
mod test {
    use crate::{*, engines::null::*};

    #[test]
    fn should_run () -> Result<()> {
        let mut app = NullWidget;
        app.handle(&mut ())?;
        app.render(&mut ())?;
        app.run(())?;
        Ok(())
    }

}
