//! Rows, columns, and layers

use crate::*;
use super::*;

/// Order multiple `Widget`s along X (columns), Y (rows), or Z (layers).
pub struct Stacked<'a, T, U>(
    /// The axis along which the components are stacked
    pub Axis,
    /// The stacked components
    pub Vec<Collected<'a, T, U>>,
);

impl<'a, T, U> std::fmt::Debug for Stacked<'a, T, U> {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Stacked({:?}, {:?})", self.0, &self.1)
    }
}

impl<'a, T, U> Stacked<'a, T, U> {

    /// Stacked left to right
    pub fn x (items: impl Fn(&mut Collector<'a, T, U>)) -> Self {
        Self(Axis::X, Collector::collect_items(items).0)
    }

    /// Stacked top to bottom
    pub fn y (items: impl Fn(&mut Collector<'a, T, U>)) -> Self {
        Self(Axis::Y, Collector::collect_items(items).0)
    }

    /// Stacked back to front
    pub fn z (items: impl Fn(&mut Collector<'a, T, U>)) -> Self {
        Self(Axis::Z, Collector::collect_items(items).0)
    }

}

pub struct Rows<'a, T, U>(Vec<Collected<'a, T, U>>);

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

pub struct Columns<'a, T, U>(Vec<Collected<'a, T, U>>);

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

pub struct Layers<'a, T, U>(Vec<Collected<'a, T, U>>);

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

    struct StackedWidget1;

    impl<T, U> Output<T, U> for StackedWidget1 {
        fn render (&self, engine: &mut T) -> Result<Option<U>> {
            Stacked::x(|add|{
                add("String");
                add(String::from("String"));
                add(Stacked::y(|add|{
                    add("String");
                    add(String::from("String"));
                    add(Stacked::z(|add|{
                        add("String");
                        add(String::from("String"));
                    }));
                }));
            }).render(engine)
        }
    }

    #[test]
    fn should_stack_callback () -> Result<()> {
        StackedWidget1.render(&mut ())?;
        Ok(())
    }

    struct StackedWidget2;

    impl<T, U> Output<T, U> for StackedWidget2 {
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

    #[test]
    fn should_stack_builder () -> Result<()> {
        StackedWidget2.render(&mut ())?;
        Ok(())
    }

}
