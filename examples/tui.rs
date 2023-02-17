use thatsit_core::{*, layouts::*, engines::tui::*};
use std::io::Write;

#[derive(Debug)]
pub struct ExampleComponent {
    label: String,
    state: String
}

impl<W: Write> Input<TUI<W>, bool> for ExampleComponent {
    fn handle (&mut self, engine: &mut TUI<W>) -> Result<Option<bool>> {
        if let Some(TUIInputEvent::Key(key)) = engine.event {
            match key.code {
                KeyCode::Esc => {
                    engine.exit()?;
                },
                KeyCode::Char(c) => {
                    self.state.push(c);
                }
                KeyCode::Backspace => {
                    self.state.pop();
                }
                _ => {}
            }
        }
        Ok(None)
    }
}

impl<W: Write> Output<TUI<W>, [u16;2]> for ExampleComponent {
    fn render (&self, engine: &mut TUI<W>) -> Result<Option<[u16;2]>> {
        Stacked::x(|add|{
            add("Press Esc to quit ");
            add(Stacked::y(|add|{
                add(&self.label);
                add(&self.state);
                add(Stacked::z(|add|{
                    add("String");
                    add(String::from("String"));
                }));
            }));
        }).render(engine)
    }
}

fn main () -> Result<()> {

    let result = ExampleComponent {
        label: "Enter some text:".to_string(),
        state: "> ".to_string()
    }.run(TUI::stdio()?)?;

    Ok(())

}
