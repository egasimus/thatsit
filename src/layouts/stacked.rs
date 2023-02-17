//! Rows, columns, and layers

use crate::*;
use super::*;

use std::marker::PhantomData;

/// Order multiple `Widget`s along X (columns), Y (rows), or Z (layers).
pub struct Stacked<'a, T, U>(
    /// The axis along which the components are stacked
    pub Axis,
    /// The stacked components
    pub Vec<Collected<'a, T, U>>,

    PhantomData<T>,
    PhantomData<U>,
);

impl<'a, T, U> std::fmt::Debug for Stacked<'a, T, U> {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Stacked({:?}, {:?})", self.0, &self.1)
    }
}

impl<'a, T, U> Stacked<'a, T, U> {

    /// Stacked left to right
    pub fn x (items: impl Fn(&mut Collector<'a, T, U>)) -> Self {
        Self(Axis::X, Collector::collect(items).0, Default::default(), Default::default())
    }

    /// Stacked top to bottom
    pub fn y (items: impl Fn(&mut Collector<'a, T, U>)) -> Self {
        Self(Axis::Y, Collector::collect(items).0, Default::default(), Default::default())
    }

    /// Stacked back to front
    pub fn z (items: impl Fn(&mut Collector<'a, T, U>)) -> Self {
        Self(Axis::Z, Collector::collect(items).0, Default::default(), Default::default())
    }

}

#[cfg(test)]
mod test {
    use crate::{*, engines::tui::TUI, layouts::Stacked};

    struct StackedWidget;

    impl<W: std::io::Write> Output<TUI<W>, [u16;2]> for StackedWidget {

        fn render (&self, engine: &mut TUI<W>) -> Result<Option<[u16;2]>> {
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
    fn should_stack () -> Result<()> {
        let widget = StackedWidget;
        Ok(())
    }
}
