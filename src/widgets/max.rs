use crate::{*, widgets::*};

use crate::engines::tui::Crossterm;

use std::io::Write;

/// Set maximum size for the contained widget across one or both axes
#[derive(Debug)]
pub enum Max<U, T> {
    X(U, T),
    Y(U, T),
    XY((U, U), T)
}

impl<'a, T> Output<Crossterm<'a>, (u16, u16)> for Max<u16, T>
where
    T: Output<Crossterm<'a>, (u16, u16)>
{
    fn render (&self, context: &mut Crossterm<'a>) -> Result<Option<(u16, u16)>> {
        self.get().render(context.area(|area|match self {
            Self::X(max_width, _) => {
                Area(area.0, area.1, area.2.min(*max_width), area.3)
            },
            Self::Y(max_height, _) => {
                Area(area.0, area.1, area.2, area.3.min(*max_height))
            },
            Self::XY((max_width, max_height), _) => {
                Area(area.0, area.1, area.2.min(*max_width), area.3.min(*max_height))
            }
        }))
    }
}

impl<Unit, T> Proxy<T> for Max<Unit, T> {
    fn get (&self) -> &T {
        match self { Self::X(_, w)  => w, Self::Y(_, w)  => w, Self::XY(_, w) => w }
    }
    fn get_mut (&mut self) -> &mut T {
        match self { Self::X(_, w)  => w, Self::Y(_, w)  => w, Self::XY(_, w) => w }
    }
}
