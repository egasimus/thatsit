use crate::{*, engines::tui::Crossterm};

impl<S: AsRef<str>> Input<String, String> for S {
    fn handle (&mut self, context: String) -> Result<Option<String>> {
        // FIXME: render the string as a prompt
        Ok(Some(context))
    }
}

impl<S: AsRef<str>> Input<crossterm::event::Event, bool> for S {
    fn handle (&mut self, context: crossterm::event::Event) -> Result<Option<bool>> {
        // FIXME: render the string as a prompt
        Ok(None)
    }
}

impl Output<String, ()> for String {
    fn render (&self, context: &mut String) -> Result<Option<()>> {
        // FIXME: render the string as a prompt
        Ok(Some(()))
    }
}

impl Output<String, ()> for &str {
    fn render (&self, context: &mut String) -> Result<Option<()>> {
        // FIXME: render the string as a prompt
        Ok(Some(()))
    }
}

impl<'a> Output<Crossterm<'a>, (u16, u16)> for String {
    fn render (&self, context: &mut Crossterm<'a>) -> Result<Option<(u16, u16)>> {
        // FIXME: render the string as a label
        Ok(Some((10, 10)))
    }
}

impl<'a> Output<Crossterm<'a>, (u16, u16)> for &str {
    fn render (&self, context: &mut Crossterm<'a>) -> Result<Option<(u16, u16)>> {
        // FIXME: render the string as a label
        Ok(Some((10, 10)))
    }
}
