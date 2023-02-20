//! # REPL engine.
//!
//! Renders the app to the terminal as a series of question/answer prompts.

use crate::*;
use std::io::{Stdout, Write, BufRead};

#[derive(Debug, PartialEq, Eq)]
pub struct Repl<R, W> {
    input:  R,
    output: W,
    pub exited: bool
}

impl<R: BufRead, W: Write> Context for Repl<R, W> {
    type Handled  = String;
    type Rendered = [u16;2];
    fn render (&mut self, engine: &impl Output<Self, Self::Rendered>) -> Result<()> {
        engine.render(self)?;
        self.output.flush()?;
        Ok(())
    }
    fn handle (&mut self, _: &mut impl Input<Self, Self::Handled>) -> Result<()> {
        self.read_line()?;
        Ok(())
    }
    fn exited (&self) -> bool {
        self.exited
    }
}

impl<R, W: Write> Repl<R, W> {
    pub fn write (&mut self, data: &[u8]) -> Result<()> {
        self.output.write_all(data)?;
        Ok(())
    }
}

impl<R: BufRead, W> Repl<R, W> {
    fn read_line (&mut self) -> Result<String> {
        let mut input = String::new();
        self.input.read_line(&mut input)?;
        Ok(input)
    }
}

impl Repl<std::io::StdinLock<'static>, Stdout> {
    /// Create a REPL context talking to the user over stdin/stdout
    pub fn stdio () -> Self {
        let input  = std::io::stdin().lock();
        let output = std::io::stdout();
        Self { input, output, exited: false }
    }
}

/// A REPL context talking to the user over stdin/stdout
pub type ReplStdio = Repl<std::io::StdinLock<'static>, Stdout>;

impl Repl<std::io::BufReader<&'static [u8]>, Vec<u8>> {
    /// Create a REPL context taking predefined input and rendering to string
    pub fn harness (input: &'static [u8]) -> Self {
        let input  = std::io::BufReader::new(input);
        let output = vec![];
        Self { input, output, exited: false }
    }
}

/// A REPL context taking predefined input and rendering to string
pub type ReplHarness = Repl<std::io::BufReader<&'static [u8]>, Vec<u8>>;

#[cfg(test)]
mod test {

    use crate::{*, engines::repl::*};
    use std::{error::Error, io::BufReader};

    #[test]
    fn repl_should_run () -> Result<()> {
        let app = String::from("just a label");
        let mut engine = ReplHarness::harness("newline\n".as_bytes());
        engine.exited = true;
        assert_eq!(app.run(engine)?.output, "just a label".as_bytes());
        Ok(())
    }

}
