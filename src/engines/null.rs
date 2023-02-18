//! # Null engine. Does nothing.

use crate::*;

struct NullWidget;

impl Input<(), ()> for NullWidget {
    fn handle (&mut self, _: &mut ()) -> Result<Option<()>> {
        Ok(Some(()))
    }
}

impl Output<(), ()> for NullWidget {
    fn render (&self, _: &mut ()) -> Result<Option<()>> {
        Ok(Some(()))
    }
}

impl<'a, X: Input<(), ()> + Output<(), ()>> Engine<()> for X {
    fn run (self, _: ()) -> Result<()> {
        Ok(())
    }
}

impl Output<(), ()> for String {
    fn render (&self, _: &mut ()) -> Result<Option<()>> {
        Ok(Some(()))
    }
}

impl Output<(), ()> for &str {
    fn render (&self, _: &mut ()) -> Result<Option<()>> {
        Ok(Some(()))
    }
}
