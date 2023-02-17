//! # TUI platform
//!
//! This platform renders an interface to a terminal
//! using `crossterm`.

use crate::*;

use ::crossterm::{
    ExecutableCommand,
    QueueableCommand,
    event::{
        poll,
        read
    },
    style::{
        ResetColor,
        SetForegroundColor,
        Color,
        Print
    },
    cursor::{
        Hide,
        Show,
        MoveTo,
    },
    terminal::{
        Clear,
        ClearType,
        EnterAlternateScreen,
        LeaveAlternateScreen,
        enable_raw_mode,
        disable_raw_mode,
        size
    },
};

pub use crossterm::event::Event as TUIInputEvent;

use std::sync::{Arc, atomic::{AtomicBool, Ordering}, mpsc::{channel, Receiver}};
use std::io::Write;
use std::thread::spawn;
use std::time::Duration;

impl<W: Write, X: Input<TUIInputEvent, bool> + Output<TUI<W>, [u16;2]>> Engine<TUI<W>> for X {

    fn run (mut self, mut context: TUI<W>) -> Result<TUI<W>> {

        context.setup()?;

        loop {
            // Render display
            context.render(&self)?;

            // Respond to user input
            if let Err(e) = self.handle(context.input.recv()?) {
                panic!("{e}");
                // TODO error handling and graceful recovery
            };

            // Repeat until done
            if context.exited() {
                break
            }
        }

        Ok(context)
    }

}

/// An instance of an app hosted by crossterm.
#[derive(Debug)]
pub struct TUI<W: Write> {
    /// Exit flag. Setting this to true terminates the main loop.
    exited: Arc<AtomicBool>,
    /// Input receiver. Receives input events from input thread.
    input: Receiver<TUIInputEvent>,
    /// Output. Terminal commands are written to this.
    pub output: W,
    /// Currently available screen area.
    pub area: [u16; 4]
}

impl<W: Write> TUI<W> {

    fn setup (&mut self) -> Result<()> {
        self.output.execute(EnterAlternateScreen)?.execute(Hide)?;
        Ok(())
    }

    pub fn cleanup (&mut self) -> Result<()> {
        self.output.execute(ResetColor)?.execute(Show)?.execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    fn exited (&self) -> bool {
        self.exited.fetch_and(true, Ordering::Relaxed)
    }

    fn render (&mut self, widget: &impl Output<Self, [u16;2]>) -> Result<()> {
        self.clear()?;
        let (w, h) = size()?;
        self.area = [0, 0, w, h];
        if let Err(error) = widget.render(self) {
            self.write_error(format!("{error}").as_str())?;
        }
        // Flush output buffer
        self.output.flush()?;
        Ok(())
    }

    /// Clear the screen
    fn clear (&mut self) -> Result<()> {
        self.output.queue(ResetColor)?.queue(Clear(ClearType::All))?.queue(Hide)?;
        Ok(())
    }

    /// Write some text to the terminal.
    pub fn put (&mut self, x: u16, y: u16, text: &str) -> Result<()> {
        self.output.queue(MoveTo(x, y))?.queue(Print(text))?;
        Ok(())
    }

    /// Write some red text to the terminal.
    fn write_error (&mut self, msg: &str) -> Result<()> {
        self.clear()?;
        self.output.queue(SetForegroundColor(Color::Red))?;
        self.put(0, 0, msg)
    }

    pub fn area (&mut self, alter_area: impl Fn(&[u16;4])->[u16;4]) -> &mut Self {
        self.area = alter_area(&self.area);
        self
    }

}

type TUIStdio = TUI<std::io::Stdout>;

impl TUIStdio {

    /// Create a TUI context for talking to the user over stdin/stdout.
    pub fn stdio () -> Result<Self> {
        let output = std::io::stdout();
        enable_raw_mode()?;
        let (tx, input) = channel::<TUIInputEvent>();
        let exited = Arc::new(AtomicBool::new(false));
        // Spawn the input thread
        let exit_input_thread = exited.clone();
        spawn(move || {
            loop {
                // Exit if flag is set
                if exit_input_thread.fetch_and(true, Ordering::Relaxed) {
                    break
                }
                // Listen for events and send them to the main thread
                if poll(Duration::from_millis(100)).is_ok() {
                    if tx.send(read().unwrap()).is_err() {
                        break
                    }
                }
            }
        });
        Ok(Self { exited, input, output, area: [0, 0, 0, 0] })
    }

}

type TUIHarness = TUI<Vec<u8>>;

impl TUIHarness {
    /// Create a TUI context that takes predefined input and renders to a buffer
    pub fn harness (input: &'static [u8]) -> Self {
        let output = vec![];
        let exited = Arc::new(AtomicBool::new(false));
        let input = Box::new(std::io::BufReader::new(input));
        let (tx, input) = channel::<TUIInputEvent>();
        Self { exited, input, output, area: [0, 0, 0, 0] }
    }
}

#[cfg(test)]
mod test {

    use crate::{Engine, layouts::*, engines::tui::TUI};
    use std::{error::Error, sync::atomic::Ordering};

    #[test]
    fn tui_should_run () -> Result<(), Box<dyn Error>> {
        let app = String::from("just a label");
        let engine = TUI::harness("newline\n".as_bytes());
        engine.exited.store(true, Ordering::Relaxed);
        assert_eq!(app.run(engine)?.output, "just a label".as_bytes());
        Ok(())
    }

}
