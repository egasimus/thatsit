//! Offset the position of a widget.

use crate::*;

/// Render the contained Widget in a sub-Area starting some distance from
/// the upper left corner of the Area that was passed.
#[derive(Copy, Clone, Default)]
pub struct Offset<Unit, T>(
    /// The horizontal offset
    pub Unit,
    /// The vertical offset
    pub Unit,
    /// The drawn widget
    pub T
);

impl<Unit, T> Proxy<T> for Offset<Unit, T> {
    fn get (&self) -> &T {
        &self.2
    }
    fn get_mut (&mut self) -> &mut T {
        &mut self.2
    }
}
