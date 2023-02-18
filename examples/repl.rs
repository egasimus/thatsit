use thatsit::{*, engines::repl::*};
use std::io::{BufRead, Write};

#[derive(Debug)]
pub struct ExampleComponent {
    done:  bool,
    label: String,
    state: String
}

impl Input<String, String> for ExampleComponent {
    fn handle (&mut self, input: &mut String) -> Result<Option<String>> {
        self.state = input.clone();
        Ok(None)
    }
}

impl<R, W: Write> Output<Repl<R, W>, [u16;2]> for ExampleComponent {
    fn render (&self, context: &mut Repl<R, W>) -> Result<Option<[u16;2]>> {
        context.write(format!("\n{:?} ", self).as_bytes())?;
        Ok(None)
    }
}

fn main () -> Result<()> {

    let stdin = std::io::stdin().lock();

    let result = ExampleComponent {
        done: false,
        label: "Enter some text to be stored".to_string(),
        state: "".to_string()
    }.run(Repl::stdio())?;

    Ok(())
}
