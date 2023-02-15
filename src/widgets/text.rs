use crate::{*, engines::tui::Crossterm};

impl<'a> Input<String, String> for String {
    fn handle (&mut self, context: String) -> Result<Option<String>> {
        // FIXME: render the string as a prompt
        Ok(Some(context))
    }
}

impl<'a> Input<String, String> for &str {
    fn handle (&mut self, context: String) -> Result<Option<String>> {
        // FIXME: render the string as a prompt
        Ok(Some(context))
    }
}

impl<'a> Output<String, ()> for String {
    fn render (&self, context: &mut String) -> Result<Option<()>> {
        // FIXME: render the string as a prompt
        Ok(Some(()))
    }
}

impl<'a> Output<String, ()> for &str {
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
