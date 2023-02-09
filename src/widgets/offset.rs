use crate::{*, widgets::{*, proxy::*}};

use crate::engines::tui::Crossterm;

use std::io::Write;

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

impl<'a, W> Output<Crossterm<'a>, (u16, u16)> for Offset<u16, W>
where
    W: Output<Crossterm<'a>, (u16, u16)>
{
    fn render (&self, context: &mut Crossterm<'a>) -> Result<Option<(u16, u16)>> {
        self.2.render(context.area(|area|Area(
            area.x() + self.0,
            area.y() + self.1,
            area.w().saturating_sub(self.0),
            area.h().saturating_sub(self.1)
        )))
    }
}

impl<Unit, T> Proxy<T> for Offset<Unit, T> {
    fn get (&self) -> &T {
        &self.2
    }
    fn get_mut (&mut self) -> &mut T {
        &mut self.2
    }
}
