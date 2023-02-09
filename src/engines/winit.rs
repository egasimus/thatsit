//! # Winit platform
//!
//! This platform renders an interface to one or more windows created with `winit`.

use crate::*;

pub type Unit = f32;

pub type WinitContext<'a> = (&'a mut Gles2Renderer, &'a mut Gles2Frame);

impl<'a, X> Engine<WinitContext<'a>> for X
where
    X: Input<WinitEvent> + Output<WinitContext<'a>>
{
}
