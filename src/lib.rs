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
