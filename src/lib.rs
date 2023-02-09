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
        type NullEngineContext = ();
        struct NullEngine;
        impl<'a, X> Engine<NullEngineContext> for X
        where
            X: Input<NullInputContext> + Output<NullOutputContext>
        {
            fn done (&self) -> bool {
                true
            }
            fn run (self, context: NullEngineContext) -> Result<Self> {
                Ok(self)
            }
        }
        struct NullWidget;
        type NullInputContext = ();
        impl Input<NullInputContext> for NullWidget {
            fn handle (self, context: NullInputContext) -> Result<Self> {
                Ok(self)
            }
        }
        type NullOutputContext = ();
        type NullOutputResult = ();
        impl Output<NullOutputContext, NullOutputResult> for NullWidget {
            fn render (self, context: &mut NullOutputContext) -> Result<Option<NullOutputResult>> {
                Ok(Some(()))
            }
        }
        NullWidget.run(())
    }
}
