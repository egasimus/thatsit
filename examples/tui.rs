use thatsit_core::{*, tui::*};

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

impl Input<Crossterm> for ExampleComponent {
    fn handle (mut self, input: String) -> Result<Self> {
        Ok(self)
    }
}

impl Output<Crossterm> for ExampleComponent {
    fn render (self, context: &mut String) -> Result<Self> {
        Ok(self)
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
