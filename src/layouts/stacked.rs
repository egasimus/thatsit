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

pub struct Rows<'a, T, U>(pub dyn Fn(&mut Collector<'a, T, U>));

pub struct Columns<'a, T, U>(pub dyn Fn(&mut Collector<'a, T, U>));

pub struct Layers<'a, T, U>(pub dyn Fn(&mut Collector<'a, T, U>));

#[cfg(test)]
mod test {

    use crate::{*, layouts::*};
    use std::io::Write;

    struct StackedWidget1;

    impl<T, U> Output<T, U> for StackedWidget1 {
        fn render (&self, engine: &mut T) -> Result<Option<U>> {
            Columns(|add|{
                add("String");
                add(String::from("String"));
                add(Rows(|add|{
                    add("String");
                    add(String::from("String"));
                    add(Layers(|add|{
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
