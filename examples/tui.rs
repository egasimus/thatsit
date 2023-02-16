use thatsit_core::{*, engines::tui::*};

#[derive(Debug)]
pub struct ExampleComponent {
    label: String,
    state: String
}

impl Input<CrosstermInputEvent, bool> for ExampleComponent {
    fn handle (&mut self, input: CrosstermInputEvent) -> Result<Option<bool>> {
        Ok(None)
    }
}

impl<'a> Output<Crossterm<'a>, [u16;2]> for ExampleComponent {
    fn render (&self, context: &mut Crossterm<'a>) -> Result<Option<[u16;2]>> {
        Ok(Some([10, 10]))
    }
}

fn main () -> Result<()> {

    let result = ExampleComponent {
        label: "Enter some text to be stored".to_string(),
        state: "".to_string()
    }.run(
        Crossterm::new(std::io::stdout())
    )?;

    Ok(())

}
