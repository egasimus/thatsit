use crate::*;

/// Moves a widget by a specified distance.
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
