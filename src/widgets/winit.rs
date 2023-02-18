use crate::{*, engines::winit::*};

impl Output<Winit, Vec<[f32;4]>> for String {
    fn render (&self, engine: &mut Winit) -> Result<Option<Vec<[f32;4]>>> {
        self.as_str().render(engine)
    }
}

impl Output<Winit, Vec<[f32;4]>> for &str {
    fn render (&self, _engine: &mut Winit) -> Result<Option<Vec<[f32;4]>>> {
        // TODO: render text
        Ok(Some(vec![[0.0, 0.0, 0.0, 0.0]]))
    }
}
