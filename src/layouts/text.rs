use crate::*;

impl<S: AsRef<str>> Input<String, String> for S {
    fn handle (&mut self, context: String) -> Result<Option<String>> {
        // FIXME: render the string as a prompt
        Ok(Some(context))
    }
}

impl Output<String, ()> for String {
    fn render (&self, _: &mut String) -> Result<Option<()>> {
        // FIXME: render the string as a prompt
        Ok(Some(()))
    }
}

impl Output<String, ()> for &str {
    fn render (&self, _: &mut String) -> Result<Option<()>> {
        // FIXME: render the string as a prompt
        Ok(Some(()))
    }
}
