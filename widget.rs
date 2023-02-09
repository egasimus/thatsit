use crate::*;

/// Shorthand for implementing the `render` method of a widget.
#[macro_export] macro_rules! impl_render {
    ($self:ident, $out:ident, $area:ident => $body:expr) => {
        fn render (&$self, $out: &mut dyn Write, $area: Area) -> Result<(Unit, Unit)> { $body }
    }
}

/// Shorthand for implementing the `handle` method of a widget.
#[macro_export] macro_rules! impl_handle {
    ($self:ident, $event:ident => $body:expr) => {
        fn handle (&mut $self, $event: &Event) -> Result<bool> {
            $body
        }
    }
}

/// An interface component. Can render itself and handle input.
pub trait Widget {
    impl_render!(self, _out, _area => Ok((0, 0)));
    impl_handle!(self, _event => Ok(false));
    /// Thanks @steffahn for suggesting this!
    fn collect <'a> (self, collect: &mut Collect<'a>) where Self: 'a + Sized {
        collect.0.push(Layout::Box(Box::new(self)));
    }
    /// Set up the terminal and run the main loop. As the main thread goes into a render loop,
    /// a separate input thread is launched, which sends input events to the main thread.
    /// After the exit flag is set, reset the terminal and exit.
    ///
    /// # Arguments
    ///
    /// * `exited` - Atomic exit flag. Setting this to `true` tells both threads to end.
    /// * `term` - A writable output, such as `std::io::stdout()`.
    fn run (self, exited: &'static AtomicBool, term: &mut dyn Write) -> Result<()>
        where Self: Sized
    {
        let app: std::cell::RefCell<Self> = RefCell::new(self);
        // Set up event channel and input thread
        let (tx, rx) = channel::<Event>();
        spawn_input_thread(tx, exited);
        // Setup terminal and panic hook
        setup(term, true)?;
        // Render app and listen for updates
        loop {
            // Clear screen
            clear(term).unwrap();
            // Break loop if exited
            if exited.fetch_and(true, Ordering::Relaxed) == true {
                break
            }
            // Render
            let (w, h) = size()?;
            if let Err(error) = app.borrow().render(term, Area(0, 0, w, h)) {
                write_error(term, format!("{error}").as_str()).unwrap();
            }
            // Flush output buffer
            term.flush().unwrap();
            // Wait for input and update
            app.borrow_mut().handle(&rx.recv().unwrap()).unwrap();
        }
        // Clean up
        teardown(term)?;
        Ok(())
    }
}

impl Debug for dyn Widget {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "dyn[Widget]")
    }
}

/// Widgets work the same when referenced.
impl<W: Widget> Widget for &W {
    impl_render!(self, out, area => (*self).render(out, area));
    impl_handle!(self, _event => unreachable!());
    fn collect <'a> (self, collect: &mut Collect<'a>) where Self: 'a + Sized {
        collect.0.push(Layout::Ref(self));
    }
}

/// Widgets work the same when boxed.
impl<'a> Widget for Box<dyn Widget + 'a> {
    impl_render!(self, out, area => (**self).render(out, area));
    impl_handle!(self, event => (**self).handle(event));
    fn collect <'b> (self, collect: &mut Collect<'b>) where Self: 'b + Sized {
        collect.0.push(Layout::Box(self));
    }
}

/// The null type is a valid widget which shows nothing and does nothing.
impl Widget for () {}

/// The number type is a valid widget representing an empty X*Y square.
impl Widget for Unit {
    impl_render!(self, _out, _area => Ok((*self, *self)));
}

/// A pair of numbers represents a rectangular spacer.
impl Widget for (Unit, Unit) {
    impl_render!(self, _out, _area => Ok((self.0, self.1)));
}

/// Widgets can be optional. Note that hiding widgets by setting them to None erases their state.
impl<W: Widget> Widget for Option<W> {
    impl_render!(self, out, area => match self {
        Some(item) => item.render(out, area),
        None => Ok((0, 0))
    });
}

/// String slices are rendered to the screen.
impl Widget for &str {
    impl_render!(self, out, area => {
        let size = (self.len() as Unit, 1);
        area.min(size)?.home(out)?.queue(Print(&self))?;
        Ok(size)
    });
}

/// Strings are rendered to the screen.
impl Widget for String {
    impl_render!(self, out, area => {
        let size = (self.len() as Unit, 1);
        area.min(size)?.home(out)?.queue(Print(&self))?;
        Ok(size)
    });
}

/// Wrapper for integration with `crossterm::StyledContent`.
pub struct Styled<'a, W: Widget + Stylize + Display>(pub &'a StyleFn<'a, W>, pub W);

/// A closure which takes a stylable thing and returns a styled thing,
/// using `crossterm::StyledContent`.
pub type StyleFn<'a, W> = dyn Fn(W) -> StyledContent<W> + 'a;

impl<'a, W: Widget + Stylize + Display + Clone> Widget for Styled<'a, W> {
    impl_render!(self, out, area => {
        let size = self.1.render(&mut Vec::<u8>::new(), area)?;
        let styled = self.0(self.1.clone());
        area.min(size)?.home(out)?.queue(Print(&styled))?;
        Ok(size)
    });
}

/// Same as `Styled` but takes a boxed closure.
pub struct StyledBoxed<'a, W: Widget + Stylize + Display>(pub Box<StyleFn<'a, W>>, pub W);

impl<'a, W: Widget + Stylize + Display + Clone> Widget for StyledBoxed<'a, W> {
    impl_render!(self, out, area => {
        let size = self.1.render(&mut Vec::<u8>::new(), area)?;
        let styled = self.0(self.1.clone());
        area.min(size)?.home(out)?.queue(Print(&styled))?;
        Ok(size)
    });
}

impl<D: Display> Widget for StyledContent<D> {
    impl_render!(self, out, area => {
        let size = (self.content().to_string().len() as Unit, 1);
        area.min(size)?.home(out)?.queue(Print(&self))?;
        Ok((1, 1))
    });
}

/// Compare render output against an expected value.
#[macro_export] macro_rules! assert_rendered {
    ($layout:ident == $expected:expr) => {
        let mut output = Vec::<u8>::new();
        assert_eq!($layout.render(&mut output, Area(Point(5, 5), Size(10, 10))).unwrap(), ());
        assert_eq!(from_utf8(&output).unwrap(), $expected);
    }
}

/// A widget with an associated action triggered on Enter or Space.
/// Combine this with a background and a border to get a button.
pub struct Link<T: Fn(&Self)->Result<bool>, U: Widget>(T, U);

impl<T: Fn(&Self)->Result<bool>, U: Widget> Widget for Link<T, U> {
    impl_render!(self, out, area => self.1.render(out, area));
    impl_handle!(self, event => Ok(match_key!((event) {
        KeyCode::Enter     => { self.0(self)? },
        KeyCode::Char(' ') => { self.0(self)? }
    })));
}

/// Generate an `Event::Key(KeyEvent { ... })` variant
#[macro_export] macro_rules! key {
    ($code:ident) => {
        crossterm::event::Event::Key(crossterm::event::KeyEvent {
            code:      crossterm::event::KeyCode::$code,
            modifiers: crossterm::event::KeyModifiers::NONE,
            kind:      crossterm::event::KeyEventKind::Press,
            state:     crossterm::event::KeyEventState::NONE
        })
    };
    ($char:literal) => {
        crossterm::event::Event::Key(crossterm::event::KeyEvent {
            code:      crossterm::event::KeyCode::Char($char),
            modifiers: crossterm::event::KeyModifiers::NONE,
            kind:      crossterm::event::KeyEventKind::Press,
            state:     crossterm::event::KeyEventState::NONE
        })
    };
    (Ctrl-$code:ident) => {
        crossterm::event::Event::Key(KeyEvent {
            code:      crossterm::event::KeyCode::$code,
            modifiers: crossterm::event::KeyModifiers::CONTROL,
            kind:      crossterm::event::KeyEventKind::Press,
            state:     crossterm::event::KeyEventState::NONE
        })
    };
    (Alt-$code:ident) => {
        crossterm::event::Event::Key(KeyEvent {
            code:      crossterm::event::KeyCode::$code,
            modifiers: crossterm::event::KeyModifiers::ALT,
            kind:      crossterm::event::KeyEventKind::Press,
            state:     crossterm::event::KeyEventState::NONE
        })
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

