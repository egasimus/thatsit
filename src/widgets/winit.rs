use crate::{*, engines::winit::*};
use std::io::{BufRead, Write};

impl Output<Winit, Vec<[f32;4]>> for String {
    fn render (&self, engine: &mut Winit) -> Result<Option<Vec<[f32;4]>>> {
        self.as_str().render(engine)
    }
}

impl Output<Winit, Vec<[f32;4]>> for &str {
    fn render (&self, engine: &mut Winit) -> Result<Option<Vec<[f32;4]>>> {
        Ok(Some(vec![[0.0, 0.0, 0.0, 0.0]]))
    }
}
