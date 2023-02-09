use crate::{*, widgets::{*, offset::Offset, layout::Layout, collect::{Collect, Collectible}}};

use std::io::Write;

/// Order multiple `Widget`s along X (columns), Y (rows), or Z (layers).
#[derive(Debug)]
pub struct Stacked<'a, T>(
    /// The axis along which the components are stacked
    pub Axis,
    /// The stacked components
    pub Vec<Layout<'a, T>>
);

impl<'a, T: Collectible> Stacked<'a, T> {
    /// Stacked left to right
    pub fn x (items: impl Fn(&mut Collect<'a, T>)) -> Self {
        Self(Axis::X, Collect::collect(items).0)
    }
    /// Stacked top to bottom
    pub fn y (items: impl Fn(&mut Collect<'a, T>)) -> Self {
        Self(Axis::Y, Collect::collect(items).0)
    }
    /// Stacked back to front
    pub fn z (items: impl Fn(&mut Collect<'a, T>)) -> Self {
        Self(Axis::Z, Collect::collect(items).0)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn should_stack () -> Result<()> {
        
    }
}
