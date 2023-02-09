use thatsit_core::{*, repl::*};

#[derive(Debug)]
pub struct ExampleComponent {
    done:  bool,
    label: String,
    state: String
}

impl Done for ExampleComponent {
    fn done (&self) -> bool {
        return self.done
    }
}

impl Input<String> for ExampleComponent {
    fn handle (mut self, input: String) -> Result<Self> {
        self.state = input;
        Ok(self)
    }
}

impl Output<String> for ExampleComponent {
    fn render (self, context: &mut String) -> Result<Self> {
        *context = format!("\n{:?} ", self);
        Ok(self)
    }
}

fn main () -> Result<()> {

    let result = ExampleComponent {
        done: false,
        label: "Enter some text to be stored".to_string(),
        state: "".to_string()
    }.run(std::io::stdin(), std::io::stdout())?;

    Ok(())
}
