//! # REPL platform
//!
//! This platform renders an interface to the terminal
//! as a series of question/answer prompts.

use crate::*;
use std::io::{Stdin, Stdout, Read, Write, BufRead};

impl<'a, X, I: BufRead, O: Write> Engine<Repl<I, O>> for X
where
    X: Input<String, String> + Output<String, ()>
{
    fn done (&self) -> bool {
        true
    }
    fn run (mut self, mut context: Repl<I, O>) -> Result<Self> {
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
        Ok(self)
    }
}

pub struct Repl<I, O> {
    input:  I,
    output: O
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

    use crate::{Engine, engines::repl::Repl};
    use std::io::BufReader;

    #[test]
    fn repl_should_be_done () {
        let app = "just a label";
        assert_eq!(Engine::<ReplHarness>::done(&app), true);
        // FIXME: The "done" flag should be a value returned by the update method of the root widget?
    }

    #[test]
    fn repl_should_run () {
        let app = "just a label";
        let engine = Repl::harness("newline\n".as_bytes());
        if let Ok(result) = app.run(engine) {
            assert_eq!(result, app);
            assert_eq!(engine.output, "just a label".as_bytes());
        } else {
            panic!("running the repl engine failed")
        }
        // FIXME: Here stdin and stdout should be replaced with general streams that just happen to
        // default to stdio; and in the test, input/output buffers should be passed.
    }

}
