//! Rows, columns, and layers

use crate::*;
use super::*;

pub struct Rows<'a, T, U>(pub(crate) Vec<Collected<'a, T, U>>);

impl<'a, T, U> Rows<'a, T, U> {
    pub fn new () -> Self {
        Self(vec![])
    }
}

impl<'a, T, U> Collection<'a, T, U> for Rows<'a, T, U> {
    /// Add a row to this collection
    fn add (&mut self, widget: Collected<'a, T, U>) -> &mut Self {
        self.0.push(widget);
        self
    }
}

pub struct Columns<'a, T, U>(pub(crate) Vec<Collected<'a, T, U>>);

impl<'a, T, U> Columns<'a, T, U> {
    pub fn new () -> Self {
        Self(vec![])
    }
}

impl<'a, T, U> Collection<'a, T, U> for Columns<'a, T, U> {
    /// Add a column to this collection
    fn add (&mut self, widget: Collected<'a, T, U>) -> &mut Self {
        self.0.push(widget);
        self
    }
}

pub struct Layers<'a, T, U>(pub(crate) Vec<Collected<'a, T, U>>);

impl<'a, T, U> Layers<'a, T, U> {
    pub fn new () -> Self {
        Self(vec![])
    }
}

impl<'a, T, U> Collection<'a, T, U> for Layers<'a, T, U> {
    /// Add a layer to this collection
    fn add (&mut self, widget: Collected<'a, T, U>) -> &mut Self {
        self.0.push(widget);
        self
    }
}

#[cfg(test)]
mod test {

    use crate::{*, layouts::*};
    use std::io::Write;

    #[test]
    fn should_stack_builder () -> Result<()> {

        struct Widget;

        impl<T, U> Output<T, U> for Widget {
            fn render (&self, engine: &mut T) -> Result<Option<U>> {
                Columns::new()
                    .add("String")
                    .add(String::from("String"))
                    .add(Rows::new()
                        .add("String")
                        .add(String::from("String"))
                        .add(Layers::new()
                            .add("String")
                            .add(String::from("String"))))
                    .render(engine)
            }
        }

        Widget.render(&mut ())?;

        Ok(())
    }

}
