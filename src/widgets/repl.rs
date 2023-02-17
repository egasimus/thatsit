use crate::{*, engines::repl::*};
use std::io::{BufRead, Write};

impl<R: BufRead, W: Write> Output<Repl<R, W>, [u16;2]> for String {
    fn render (&self, engine: &mut Repl<R, W>) -> Result<Option<[u16;2]>> {
        self.as_str().render(engine)
    }
}

impl<R: BufRead, W: Write> Output<Repl<R, W>, [u16;2]> for &str {
    fn render (&self, engine: &mut Repl<R, W>) -> Result<Option<[u16;2]>> {
        engine.write(self.as_bytes())?;
        Ok(Some([self.len() as u16, 1]))
    }
}
