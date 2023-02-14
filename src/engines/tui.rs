//! # TUI platform
//!
//! This platform renders an interface to a terminal
//! using `crossterm`.

use std::sync::mpsc::{channel, Receiver};
use std::sync::atomic::{AtomicBool, Ordering};
use std::io::Write;
use std::thread::spawn;
use std::time::Duration;
use crate::{*, widgets::*};
use crossterm::{
    ExecutableCommand,
    QueueableCommand,
    event::{
        Event,
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

pub use crossterm::event::Event as CrosstermInputEvent;

/// Exit flag. Setting this to true terminates the main loop.
static EXITED: AtomicBool = AtomicBool::new(false);

/// An instance of an app hosted by crossterm.
pub struct Crossterm<'a> {
    terminal: Box<dyn Write + 'a>,
    pub area: Rect<2, u16>,
}

impl<'a, X> Engine<Crossterm<'a>> for X
where
    X: Input<Event, bool> + Output<Crossterm<'a>, (u16, u16)>
{
    fn done (&self) -> bool {
        false
    }
    fn run (mut self, mut context: Crossterm<'a>) -> Result<Self> {
        context.setup_output()?;
        let rx = context.setup_input();
        let state = &mut self;
        loop {
            if context.exited() {
                break
            }
            // Respond to user input
            if let Err(e) = state.render(&mut context) {
                panic!("{e}");
                // TODO error handling and graceful recovery
            }
            // Render display
            if let Err(e) = state.handle(rx.recv()?) {
                panic!("{e}");
            };
        }
        Ok(self)
    }
}

impl<'a> Crossterm<'a> {

    pub fn new <T: Write + 'a> (output: T) -> Self {
        Self {
            area:     Rect::default(),
            terminal: Box::new(output),
        }
    }

    /// Spawns the input thread, which passes input events over a `mpsc::channel` into the render
    /// thread. Only stops when the exit flag is set.
    fn setup_input (&self) -> Receiver<Event> {
        let (tx, rx) = channel::<Event>();
        spawn(move || {
            loop {
                if EXITED.fetch_and(true, Ordering::Relaxed) { break }
                if poll(Duration::from_millis(100)).is_ok() {
                    if tx.send(read().unwrap()).is_err() { break }
                }
            }
        });
        rx
    }

    fn setup_output (&mut self) -> Result<()> {
        self.terminal
            .execute(EnterAlternateScreen)?
            .execute(Hide)?;
        enable_raw_mode()?;
        Ok(())
    }

    fn clear (&mut self) -> Result<()> {
        self.terminal
            .queue(ResetColor)?
            .queue(Clear(ClearType::All))?
            .queue(Hide)?;
        Ok(())
    }

    fn exited (&self) -> bool {
        EXITED.fetch_and(true, Ordering::Relaxed)
    }

    fn render <O: Output<Self, (u16, u16)>> (
        &'a mut self,
        output: &mut O
    ) -> Result<()> {
        self.clear()?;
        let (w, h) = size()?;
        self.area = (0, 0, w, h).into();
        if let Err(error) = output.render(self) {
            self.write_error(format!("{error}").as_str())?;
        }
        // Flush output buffer
        self.terminal
            .flush()
            .unwrap();
        Ok(())
    }

    fn cleanup (&mut self) -> Result<()> {
        // Clean up
        self.terminal
            .execute(ResetColor)?
            .execute(Show)?
            .execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    /// Write some text to the terminal.
    fn write_text (&mut self, x: u16, y: u16, text: &str) -> Result<()> {
        self.terminal.execute(MoveTo(x, y))?.execute(Print(text))?;
        Ok(())
    }

    /// Write some red text to the terminal.
    fn write_error (&mut self, msg: &str) -> Result<()> {
        self.clear()?;
        self.terminal.queue(SetForegroundColor(Color::Red))?;
        self.write_text(0, 0, msg)
    }

    pub fn area (&mut self, alter_area: impl Fn(&Rect<2, u16>)->Rect<2, u16>) -> &mut Self {
        self.area = alter_area(&self.area);
        self
    }

}
