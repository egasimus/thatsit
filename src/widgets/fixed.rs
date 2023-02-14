use crate::{*, widgets::*};

use crate::engines::tui::Crossterm;

#[derive(Debug)]
/// Set exact size
pub enum Fixed<Unit, W> {
    /// The contained widget will have a fixed horizontal size
    X(Unit, W),
    /// The contained widget will have a fixed vertical size
    Y(Unit, W),
    /// The contained widget will have a fixed size along both axes
    XY((Unit, Unit), W),
}

impl<'a, W> Output<Crossterm<'a>, (u16, u16)> for Fixed<u16, W>
where
    W: Output<Crossterm<'a>, (u16, u16)>
{
    fn render (&self, context: &mut Crossterm<'a>) -> Result<Option<(u16, u16)>> {
        self.get().render(context.area(|area|match self {
            Self::X(width, _)            => (area.x(), area.y(), *width, area.h()).into(),
            Self::Y(height, _)           => (area.x(), area.y(), area.w(), *height).into(),
            Self::XY((width, height), _) => (area.x(), area.y(), *width, *height).into()
        }))
    }
}

impl<Unit, T> Proxy<T> for Fixed<Unit, T> {
    fn get (&self) -> &T {
        match self { Self::X(_, w)  => w, Self::Y(_, w)  => w, Self::XY(_, w) => w }
    }
    fn get_mut (&mut self) -> &mut T {
        match self { Self::X(_, w)  => w, Self::Y(_, w)  => w, Self::XY(_, w) => w }
    }
}
