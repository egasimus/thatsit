use crate::{*, layouts::*};

impl Output<(), ()> for String {
    fn render (&self, engine: &mut ()) -> Result<Option<()>> {
        self.as_str().render(engine)
    }
}

impl Output<(), ()> for &str {
    fn render (&self, engine: &mut ()) -> Result<Option<()>> {
        Ok(None)
    }
}

impl<T: Output<(), ()>> Output<(), ()> for Fixed<u16, T> {
    fn render (&self, engine: &mut ()) -> Result<Option<()>> {
        Ok(None)
    }
}

impl<T: Output<(), ()>> Output<(), ()> for Max<u16, T> {
    fn render (&self, engine: &mut ()) -> Result<Option<()>> {
        Ok(None)
    }
}

impl<T: Output<(), ()>> Output<(), ()> for Min<u16, T> {
    fn render (&self, engine: &mut ()) -> Result<Option<()>> {
        Ok(None)
    }
}

impl<T: Output<(), ()>> Output<(), ()> for Offset<u16, T> {
    fn render (&self, engine: &mut ()) -> Result<Option<()>> {
        Ok(None)
    }
}

impl<'a> Output<(), ()> for Rows<'a, (), ()> {
    fn render (&self, engine: &mut ()) -> Result<Option<()>> {
        Ok(None)
    }
}

impl<'a> Output<(), ()> for Columns<'a, (), ()> {
    fn render (&self, engine: &mut ()) -> Result<Option<()>> {
        Ok(None)
    }
}

impl<'a> Output<(), ()> for Layers<'a, (), ()> {
    fn render (&self, engine: &mut ()) -> Result<Option<()>> {
        Ok(None)
    }
}
