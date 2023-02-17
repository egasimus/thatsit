use thatsit_core::{*, layouts::*, engines::tui::*};
use std::io::Write;

#[derive(Debug)]
pub struct ExampleComponent {
    label: String,
    state: String
}

impl Input<TUIInputEvent, bool> for ExampleComponent {
    fn handle (&mut self, input: TUIInputEvent) -> Result<Option<bool>> {
        Ok(None)
    }
}

impl<W: Write> Output<TUI<W>, [u16;2]> for ExampleComponent {
    fn render (&self, context: &mut TUI<W>) -> Result<Option<[u16;2]>> {
        Stacked::y(|add|{
            add(&self.label);
            add(&self.state);
        }).render(context)
    }
}

fn main () -> Result<()> {

    let result = ExampleComponent {
        label: "Enter some text to be stored".to_string(),
        state: "".to_string()
    }.run(TUI::stdio())?;

    Ok(())

}
