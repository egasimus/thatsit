//! # REPL platform
//!
//! This platform renders an interface to the terminal
//! as a series of question/answer prompts.

use crate::*;
use std::io::{Stdout, Write, BufRead};

impl<'a, X, R: BufRead, W: Write> Engine<Repl<R, W>> for X
where
    X: Input<String, String> + Output<String, ()>
{
    fn run (mut self, mut context: Repl<R, W>) -> Result<Repl<R, W>> {
        let state = &mut self;
        loop {
            let mut output_data = String::new();
            state.render(&mut output_data)?;
            context.output.write_all(output_data.as_bytes())?;
            context.output.flush()?;
            let mut input_data = String::new();
            context.input.read_line(&mut input_data)?;
            if state.handle(input_data)?.is_some() {
                break
            }
        }
        Ok(context)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Repl<R, W> {
    input:  R,
    output: W
}

impl Repl<std::io::StdinLock<'static>, Stdout> {
    /// Create a REPL context talking to the user over stdin/stdout
    pub fn stdio () -> Self {
        let input  = std::io::stdin().lock();
        let output = std::io::stdout();
        Self { input, output }
    }
}

/// A REPL context talking to the user over stdin/stdout
pub type ReplStdio = Repl<std::io::StdinLock<'static>, Stdout>;

impl Repl<std::io::BufReader<&'static [u8]>, Vec<u8>> {
    /// Create a REPL context taking predefined input and rendering to string
    pub fn harness (input: &'static [u8]) -> Self {
        let input  = std::io::BufReader::new(input);
        let output = vec![];
        Self { input, output }
    }
}

/// A REPL context taking predefined input and rendering to string
pub type ReplHarness = Repl<std::io::BufReader<&'static [u8]>, Vec<u8>>;

#[cfg(test)]
mod test {

    use crate::{Engine, engines::repl::ReplHarness};
    use std::{error::Error, io::BufReader};

    #[test]
    fn repl_should_run () -> Result<(), Box<dyn Error>> {
        let app = "just a label";
        let engine = ReplHarness::harness("newline\n".as_bytes());
        assert_eq!(app.run(engine)?.output, "just a label".as_bytes());
        Ok(())
    }

}
