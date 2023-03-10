//! # TUI engine
//!
//! Renders the app to a terminal as an interactive text-based GUI (TUI).

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
        SetBackgroundColor,
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

pub use crossterm::{self, event::{
    Event as TUIInputEvent,
    KeyEvent,
    KeyCode,
    KeyModifiers
}};

use std::sync::{Arc, atomic::{AtomicBool, Ordering}, mpsc::{channel, Sender, Receiver}};
use std::io::Write;
use std::thread::spawn;
use std::time::Duration;

/// An instance of an app hosted by crossterm.
#[derive(Debug)]
pub struct TUI<W: Write> {
    /// Exit flag. Setting this to true terminates the main loop.
    exited: Arc<AtomicBool>,
    /// Input receiver. Receives input events from input thread.
    input: Receiver<TUIInputEvent>,
    /// Currently handled input event
    pub event: Option<TUIInputEvent>,
    /// Output. Terminal commands are written to this.
    pub output: W,
    /// Currently available screen area.
    pub area: [u16; 4]
}

impl<W: Write> Context for TUI<W> {
    type Handled  = bool;
    type Rendered = [u16;2];

    fn setup (&mut self) -> Result<()> {
        self.output.execute(EnterAlternateScreen)?.execute(Hide)?;
        Ok(())
    }

    fn handle (&mut self, widget: &mut impl Input<Self, bool>) -> Result<()> {
        self.event = Some(self.input.recv()?);
        widget.handle(self)?;
        Ok(())
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

    fn exited (&self) -> bool {
        self.exited.fetch_and(true, Ordering::Relaxed)
    }
}

impl<W: Write> TUI<W> {

    pub fn cleanup (&mut self) -> Result<()> {
        self.output.execute(ResetColor)?.execute(Show)?.execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn exit (&mut self) -> Result<()> {
        self.exited.store(true, Ordering::Relaxed);
        self.cleanup()?;
        Ok(())
    }

    /// Clear the screen
    fn clear (&mut self) -> Result<()> {
        self.output.queue(ResetColor)?.queue(Clear(ClearType::All))?.queue(Hide)?;
        Ok(())
    }

    /// Write some text to the terminal.
    pub fn put (&mut self, x: u16, y: u16, text: &impl std::fmt::Display) -> Result<&mut Self> {
        self.output.queue(MoveTo(x, y))?.queue(Print(text))?;
        Ok(self)
    }

    pub fn set_colors (&mut self, fg: &Option<Color>, bg: &Option<Color>) -> Result<&mut Self> {
        self.output.queue(ResetColor)?;
        if let Some(fg) = fg {
            self.output.queue(SetForegroundColor(*fg))?;
        }
        if let Some(bg) = bg {
            self.output.queue(SetBackgroundColor(*bg))?;
        }
        Ok(self)
    }

    /// Write some red text to the terminal.
    fn write_error (&mut self, msg: &str) -> Result<&mut Self> {
        self.clear()?;
        self.output.queue(SetForegroundColor(Color::Red))?;
        self.put(0, 0, &msg)
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
        Ok(Self { exited, input, event: None, output, area: [0, 0, 0, 0] })
    }

}

type TUIHarness = TUI<Vec<u8>>;

impl TUIHarness {
    /// Create a TUI context that takes predefined input and renders to a buffer
    pub fn harness () -> (Self, Sender<TUIInputEvent>) {
        let output = vec![];
        let exited = Arc::new(AtomicBool::new(false));
        let (tx, input) = channel::<TUIInputEvent>();
        (Self { exited, input, event: None, output, area: [0, 0, 0, 0] }, tx)
    }
}

/// Match an input event against a specified key event
#[macro_export] macro_rules! if_key {
    ($event:expr => $code:ident => $block:block) => {
        if $event == &key!($code) {
            $block
        } else {
            false
        }
    }
}

/// Match an input event against a list of key events
#[macro_export] macro_rules! match_key {
    (($event:expr) { $($code:expr => $block:block),+ }) => {
        {
            if let Event::Key(event) = $event {
                $(if event.code == $code $block else)* { false }
            } else {
                false
            }
        }
    }
}

#[cfg(test)]
mod test {

    use crate::{MainLoop, layouts::*, engines::tui::*};
    use std::{error::Error, sync::atomic::Ordering};

    #[test]
    fn tui_should_run () -> Result<()> {
        let app = String::from("just a label");
        let (mut engine, sender) = TUI::harness();
        engine.exit(); // run once then exit
        for key in "newline\n".chars() {
            let key = KeyEvent::new(KeyCode::Char(key), KeyModifiers::empty());
            sender.send(TUIInputEvent::Key(key))?;
        }
        let output = String::from_utf8(app.run(engine)?.output)?;
        //let prefix = "\u{1b}[?1049h\u{1b}[?25l\u{1b}[0m\u{1b}[2J\u{1b}[?25l\u{1b}[1;1H";
        let prefix = "\u{1b}[0m\u{1b}[?25h\u{1b}[?1049l\u{1b}[?1049h\u{1b}[?25l\u{1b}[0m\u{1b}[2J\u{1b}[?25l\u{1b}[1;1H";
        assert_eq!(output, format!("{prefix}just a label"));
        Ok(())
    }

}
