use crate::{*, engines::tui::Crossterm};

impl<'a> Output<Crossterm<'a>, (u16, u16)> for String {
    fn render (&self, context: &mut Crossterm<'a>) -> Result<Option<(u16, u16)>> {
        Ok(Some((10, 10)))
    }
}

impl crate::widgets::collect::Collectible for String {}

impl<'a> Output<Crossterm<'a>, (u16, u16)> for &str {
    fn render (&self, context: &mut Crossterm<'a>) -> Result<Option<(u16, u16)>> {
        Ok(Some((10, 10)))
    }
}

impl crate::widgets::collect::Collectible for &str {}
