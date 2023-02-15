//! # REPL platform
//!
//! This platform renders an interface to the terminal
//! as a series of question/answer prompts.

use crate::*;
use std::io::{Stdin, Stdout, Read, Write, BufRead};

pub type ReplContext = (dyn Read, dyn Write);

impl<'a, X, I: BufRead, O: Write> Engine<(I, O)> for X
where
    X: Input<String, String> + Output<String, ()>
{
    fn done (&self) -> bool {
        true
    }
    fn run (mut self, (mut input, mut output): (I, O)) -> Result<Self> {
        let state = &mut self;
        loop {
            let mut output_data = String::new();
            state.render(&mut output_data)?;
            output.write_all(output_data.as_bytes())?;
            output.flush()?;
            let mut input_data = String::new();
            input.read_line(&mut input_data)?;
            if state.handle(input_data)?.is_some() {
                break
            }
        }
        Ok(self)
    }
}

#[cfg(test)]
mod test {

    use crate::Engine;

    #[test]
    fn repl_should_be_done () {
        let app: Engine<_, _> = "just a label";
        assert_eq!(app.done(), true);
        // FIXME: The "done" flag should be a value returned by the update method of the root widget?
    }

    #[test]
    fn repl_should_run () {
        let app = "just a label";
        let input = BufReader::new(Vec::<u8>::from("newline\n"));
        let mut output = Vec::<u8>::new();
        if let Ok(result) = app.run((input, output)) {
            assert_eq!(result, app)
        } else {
            panic!("running the repl engine failed")
        }
        // FIXME: Here stdin and stdout should be replaced with general streams that just happen to
        // default to stdio; and in the test, input/output buffers should be passed.
    }
}
