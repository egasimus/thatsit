use thatsit_core::{*, engines::repl::*};

#[derive(Debug)]
pub struct ExampleComponent {
    done:  bool,
    label: String,
    state: String
}

impl Input<String, String> for ExampleComponent {
    fn handle (&mut self, input: String) -> Result<Option<String>> {
        self.state = input.clone();
        Ok(Some(input))
    }
}

impl Output<String, ()> for ExampleComponent {
    fn render (&self, context: &mut String) -> Result<Option<()>> {
        *context = format!("\n{:?} ", self);
        Ok(None)
    }
}

fn main () -> Result<()> {

    let result = ExampleComponent {
        done: false,
        label: "Enter some text to be stored".to_string(),
        state: "".to_string()
    }.run((std::io::stdin(), std::io::stdout()))?;

    Ok(())
}
