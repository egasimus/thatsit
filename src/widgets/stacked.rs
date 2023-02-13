use crate::{*, widgets::{*, offset::Offset, collect::{Collector, Collectible, Collected}}};

use crate::engines::tui::Crossterm;

use std::io::Write;

/// Order multiple `Widget`s along X (columns), Y (rows), or Z (layers).
#[derive(Debug)]
pub struct Stacked<'a, T: Collectible>(
    /// The axis along which the components are stacked
    pub Axis,
    /// The stacked components
    pub Vec<Collected<'a, T>>
);

impl<'a, T: Collectible> Stacked<'a, T> {

    /// Stacked left to right
    pub fn x (items: impl Fn(&mut Collector<'a, T>)) -> Self {
        Self(Axis::X, Collector::collect(items).0)
    }

    /// Stacked top to bottom
    pub fn y (items: impl Fn(&mut Collector<'a, T>)) -> Self {
        Self(Axis::Y, Collector::collect(items).0)
    }

    /// Stacked back to front
    pub fn z (items: impl Fn(&mut Collector<'a, T>)) -> Self {
        Self(Axis::Z, Collector::collect(items).0)
    }

}

impl<'a, T: Collectible + Output<Crossterm<'a>, (u16, u16)>> Output<Crossterm<'a>, (u16, u16)> for Stacked<'a, T> {
    fn render (&self, context: &mut Crossterm<'a>) -> Result<Option<(u16, u16)>> {
        let mut x = 0;
        let mut y = 0;
        //match self.0 {
            //Axis::X => {
                //context.area.expect_min((self.1.len() as u16, 1))?; // FIXME height
                //for item in self.1.iter() {
                    //let (w, h) = Offset(x, 0, item).render(context)?.unwrap_or((0, 0));
                    //x = x + w;
                    //y = y.max(h);
                //}
            //},
            //Axis::Y => {
                //context.area.expect_min((1, self.1.len() as u16))?; // FIXME width
                //for item in self.1.iter() {
                    //let (w, h) = Offset(0, y, item).render(context)?.unwrap_or((0, 0));
                    //x = x.max(w);
                    //y = y + h;
                //}
            //},
            //Axis::Z => {
                //context.area.expect_min((1, 1 as u16))?; // FIXME size
                //for item in self.1.iter().rev() {
                    //let (w, h) = item.render(context)?.unwrap_or((0, 0));
                    //x = x.max(w);
                    //y = y.max(h);
                //}
            //}
        //};
        Ok(Some((x, y)))
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn should_stack () -> Result<()> {
        unimplemented!()
    }
}
