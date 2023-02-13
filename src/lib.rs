#![feature(fn_traits, unboxed_closures)]

pub mod engines;

pub mod widgets;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Engine<T> {
    fn run (self, context: T) -> Result<Self> where Self: Sized;
    fn done (&self) -> bool;
}

pub trait Input<T, U> {
    fn handle (&mut self, context: T) -> Result<Option<U>>;
}

pub trait Output<T, U> {
    fn render (&self, context: &mut T) -> Result<Option<U>>;
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test] fn should_work () -> Result<()> {

        struct NullWidget;

        impl Input<(), ()> for NullWidget {
            fn handle (&mut self, context: ()) -> Result<Option<()>> {
                Ok(Some(()))
            }
        }

        NullWidget.handle(())?;

        impl Output<(), ()> for NullWidget {
            fn render (&self, context: &mut ()) -> Result<Option<()>> {
                Ok(Some(()))
            }
        }

        NullWidget.render(&mut ())?;

        struct NullEngine;

        impl<'a, X: Input<(), ()> + Output<(), ()>> Engine<&mut ()> for X {
            fn done (&self) -> bool {
                true
            }
            fn run (self, context: &mut ()) -> Result<Self> {
                Ok(self)
            }
        }

        NullWidget.run(&mut ())?;

        Ok(())

    }
}
