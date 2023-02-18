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
        Self(Axis::X, Collector::collect(items).0)
    }

    /// Stacked top to bottom
    pub fn y (items: impl Fn(&mut Collector<'a, T, U>)) -> Self {
        Self(Axis::Y, Collector::collect(items).0)
    }

    /// Stacked back to front
    pub fn z (items: impl Fn(&mut Collector<'a, T, U>)) -> Self {
        Self(Axis::Z, Collector::collect(items).0)
    }

}

pub struct Rows<'a, T, U>(pub dyn Fn(&mut Collector<'a, T, U>));

pub struct Columns<'a, T, U>(pub dyn Fn(&mut Collector<'a, T, U>));

pub struct Layers<'a, T, U>(pub dyn Fn(&mut Collector<'a, T, U>));

#[cfg(test)]
mod test {

    use crate::{*, engines::tui::TUI, layouts::*};

    struct StackedWidget;

    impl<W: std::io::Write> Output<TUI<W>, [u16;2]> for StackedWidget {

        fn render (&self, engine: &mut TUI<W>) -> Result<Option<[u16;2]>> {

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
    fn should_stack () -> Result<()> {
        let widget = StackedWidget;
        Ok(())
    }

    #[test]
    fn should_stack_builder () -> Result<()> {
        Columns::new()
            .add("String")
            .add(String::from("String"))
            .add(Rows::new()
                .add("String")
                .add(String::from("String"))
                .add(Layers::new()
                    .add("String")
                    .add(String::from("String")))).render(engine)?;
        Ok(())
    }

    #[test]
    fn should_stack_array () -> Result<()> {
        let list = [ "foo", "bar", "baz" ];
        list.into::<Columns>().render()?;
        list.into::<Rows>().render()?;
        list.into::<Layers>().render()?;
        (list as &dyn Stacked<Axis::X, _, _>).render(())?;
        (list as &dyn Stacked<Axis::Y, _, _>).render(())?;
        (list as &dyn Stacked<Axis::Z, _, _>).render(())?;
        Ok(())
    }

}
