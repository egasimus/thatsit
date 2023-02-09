//! # REPL platform
//!
//! This platform renders an interface to the terminal
//! as a series of question/answer prompts.

use crate::*;
use std::io::{Stdin, Stdout, Write};

pub type ReplContext = (Stdin, Stdout);

impl<X> Engine<ReplContext> for X
where
    X: Input<String, String> + Output<String, String>
{
    fn done (&self) -> bool {
        true
    }
    fn run (mut self, (stdin, mut stdout): ReplContext) -> Result<Self> {
        let state = &mut self;
        loop {
            let mut output = String::new();
            state.render(&mut output)?;
            stdout.write_all(output.as_bytes())?;
            stdout.flush()?;
            let mut input = String::new();
            stdin.read_line(&mut input)?;
            state.handle(input)?;
            if state.done() {
                break
            }
        }
        Ok(self)
    }
}
