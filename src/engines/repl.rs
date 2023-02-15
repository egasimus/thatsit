//! # REPL platform
//!
//! This platform renders an interface to the terminal
//! as a series of question/answer prompts.

use crate::*;
use std::io::{Stdin, Stdout, Write};

pub type ReplContext = (Stdin, Stdout);

impl<X> Engine<ReplContext> for X
where
    X: Input<String, String> + Output<String, ()>
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

#[cfg(test)]
mod test {

    use crate::Engine;

    #[test]
    fn repl_should_be_done () {
        let app = "just a label";
        assert_eq!(app.done(), true);
        // FIXME: The "done" flag should be a value returned by the update method of the root widget?
    }

    //#[test]
    //fn repl_should_run () {
        //let app = "just a label";
        //if let Ok(result) = app.run((std::io::stdin(), std::io::stdout())) {
            //assert_eq!(result, app)
        //} else {
            //panic!("running the repl engine failed")
        //}
        //// FIXME: Here stdin and stdout should be replaced with general streams that just happen to
        //// default to stdio; and in the test, input/output buffers should be passed.
    //}
}
