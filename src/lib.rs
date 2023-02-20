#![feature(fn_traits, unboxed_closures)]

pub mod engines;
pub mod layouts;
pub mod widgets;

mod input;
pub use input::*;

mod output;
pub use output::*;

/// Standard result type. Shorthand for `Result<T, Box<dyn std::error::Error>>`
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Provides the entry point into the UI's main event loop.
pub trait MainLoop<T> {
    fn run (self, context: T) -> Result<T>;
}

/// Trait for standard implementations of main loop.
pub trait Context {
    type Handled;
    type Rendered;

    fn setup (&mut self) -> Result<()> {
        Ok(())
    }

    fn handle (&mut self, _: &mut impl Input<Self, Self::Handled>)
        -> Result<()> where Self: Sized;

    fn render (&mut self, _: &impl Output<Self, Self::Rendered>)
        -> Result<()> where Self: Sized;

    fn exited (&self) -> bool;

}

impl<X, A> MainLoop<A> for X where
    A: Context,
    X: Input<A, A::Handled> + Output<A, A::Rendered>
{
    fn run (mut self, mut context: A) -> Result<A> {
        context.setup()?;
        loop {
            context.render(&self)?;
            context.handle(&mut self)?;
            if context.exited() {
                break
            }
        }
        Ok(context)
    }
}

pub trait Widget<A, B, C>: Input<A, B> + Output<A, C> {}

impl<X, A, B, C> Widget<A, B, C> for X where X: Input<A, B> + Output<A, C> {}

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
