//! # Winit engine
//!
//! Renders an interface to one or more windows created with `winit`.

use crate::{*, layouts::*};

use slog::{Drain, debug, warn, crit};

use std::{
    rc::Rc,
    cell::{Cell, RefCell, RefMut},
    sync::{Arc, atomic::{AtomicBool, Ordering}},
    time::{Instant},
    collections::HashMap,
};

use smithay::{
    output::{PhysicalProperties, Subpixel, Mode as OutputMode},
    backend::{
        egl::{
            Error as EGLError, EGLContext, EGLSurface,
            native::XlibWindow,
            context::GlAttributes,
            display::EGLDisplay
        },
        renderer::{Bind},
        winit::{WindowSize, WinitVirtualDevice,}
    },
    utils::{Size, Physical}
};

use winit::{
    dpi::{LogicalSize, LogicalPosition},
    event::{MouseButton, MouseScrollDelta, Event, WindowEvent, ElementState, KeyboardInput, Touch, TouchPhase},
    event_loop::{ControlFlow, EventLoopBuilder, EventLoop},
    platform::{unix::{WindowExtUnix, EventLoopBuilderExtUnix}, run_return::EventLoopExtRunReturn},
    window::{WindowId, WindowBuilder, Window as WinitWindow},
};

pub type Unit = f32;

impl<'a, X> Engine<Winit> for X
where
    X: Input<WinitEvent, bool> + Output<Winit, Vec<[f32;4]>>
{
    fn run (mut self, mut context: Winit) -> Result<Winit> {
        context.setup();
        loop {
            context.render(&self)?;     // Render display
            context.handle(&mut self)?; // Respond to user input
            if context.exited() {       // Repeat until done
                break
            }
        }
        Ok(context)
    }
}

pub struct Winit {
    logger:        slog::Logger,
    _log_guard:    slog_scope::GlobalLoggerGuard,
    running:       Arc<AtomicBool>,
    started:       Cell<Option<Instant>>,
    windows:       Rc<RefCell<HashMap<WindowId, WinitHostWindow>>>,
    events:        Rc<RefCell<EventLoop<()>>>,
    egl_context:   smithay::backend::egl::EGLContext,
    egl_display:   smithay::backend::egl::display::EGLDisplay,
    renderer:      Rc<RefCell<smithay::backend::renderer::gles2::Gles2Renderer>>,
}

impl Winit {

    pub fn new () -> Result<Self> {
        Self::init(EventLoop::new())
    }

    pub fn harness () -> Result<Self> {
        Self::init(EventLoopBuilder::new().with_any_thread(true).build())
    }

    /// Initialize winit engine
    fn init (events: EventLoop<()>) -> Result<Self> {

        // Create the logger
        let fused = slog_async::Async::default(slog_term::term_full().fuse()).fuse();
        let logger = slog::Logger::root(fused, slog::o!(),);
        let log_guard = slog_scope::set_global_logger(logger.clone());
        slog_stdlog::init().expect("Could not setup log backend");
        debug!(&logger, "Logger initialized");
        debug!(&logger, "Starting Winit engine");

        // Create a null window to host the EGLDisplay
        let window = std::sync::Arc::new(WindowBuilder::new()
            .with_inner_size(LogicalSize::new(16, 16))
            .with_title("Charlie Null")
            .with_visible(false)
            .build(&events)?);

        // Create the EGL context and renderer
        let egl_display = EGLDisplay::new(window, logger.clone()).unwrap();
        let egl_context = EGLContext::new_with_config(&egl_display, GlAttributes {
            version: (3, 0), profile: None, vsync: true, debug: cfg!(debug_assertions),
        }, Default::default(), logger.clone())?;
        let renderer = make_renderer(&logger, &egl_context)?;

        Ok(Self {
            logger: logger.clone(),
            _log_guard: log_guard,
            renderer: Rc::new(RefCell::new(renderer)),
            events:   Rc::new(RefCell::new(events)),
            windows:  Rc::new(RefCell::new(HashMap::new())),
            running:  Arc::new(AtomicBool::new(true)),
            started:  Cell::new(None),
            egl_display,
            egl_context,
        })

    }

    fn setup (&mut self) {
        self.started.set(Some(Instant::now()));
    }

    pub fn exit (&self) {
        self.running.store(false, Ordering::Relaxed)
    }

    pub fn exited (&self) -> bool {
        !self.running.fetch_and(true, Ordering::Relaxed)
    }

    /// For each host window, render the corresponding view of the app
    fn render (&mut self, app: &impl Output<Winit, Vec<[f32;4]>>) -> Result<()> {
        if let Err(e) = {
            let windows = self.windows.clone();
            for (_, output) in windows.borrow().iter() {
                if let Some(size) = output.resized.take() {
                    output.surface.resize(size.w, size.h, 0, 0);
                }
                self.renderer().bind(output.surface.clone())?;
                let size = output.surface.get_size().unwrap();
                //self.render(&output.output, &size, output.screen)?;
                app.render(self)?;
                output.surface.swap_buffers(None)?;
            }
            Ok(())
        } {
            crit!(self.logger, "Render error: {e}");
            Err(e)
        } else {
            Ok(())
        }
    }

    fn handle (&mut self, app: &mut impl Input<WinitEvent, bool>) -> Result<()> {
        if let Err(e) = {
            let mut closed = false;
            let events = self.events.clone();
            events.borrow_mut().run_return(|event, _target, control_flow| {
                //debug!(self.logger, "{target:?}");
                match event {
                    Event::RedrawEventsCleared => {
                        *control_flow = ControlFlow::Exit;
                    }
                    Event::RedrawRequested(_id) => {
                        //callback(0, WinitEvent::Refresh);
                    }
                    Event::WindowEvent { window_id, event } => {
                        closed = self.window_event(&window_id, event)
                    }
                    _ => {}
                }
            });
            if closed {
                Err(WinitHostError::WindowClosed.into())
            } else {
                Ok(())
            }
        } {
            crit!(self.logger, "Update error: {e}");
            Err(e)
        } else {
            Ok(())
        }
    }

    fn renderer (&self) -> RefMut<smithay::backend::renderer::gles2::Gles2Renderer> {
        self.renderer.borrow_mut()
    }

    pub fn window_add (&self, window: WinitHostWindow) -> () {
        let window_id = window.id();
        self.windows.borrow_mut().insert(window_id, window);
    }

    pub fn window_event <'b> (&self, window_id: &WindowId, event: WindowEvent<'b>) -> bool {
        match self.windows.borrow().get(window_id) {
            Some(window) => {
                let duration = Instant::now().duration_since(self.started.get().unwrap());
                let nanos    = duration.subsec_nanos() as u64;
                let time     = ((1000 * duration.as_secs()) + (nanos / 1_000_000)) as u64;
                let result   = match event {

                    WindowEvent::CloseRequested |
                    WindowEvent::Destroyed      |
                    WindowEvent::Resized(_)     |
                    WindowEvent::Focused(_)     |
                    WindowEvent::ScaleFactorChanged { .. }
                        => Self::on_window(time, window, event),

                    WindowEvent::KeyboardInput { .. }
                        => Self::on_keyboard(time, window, event),

                    WindowEvent::CursorMoved { .. } |
                    WindowEvent::MouseWheel  { .. } |
                    WindowEvent::MouseInput  { .. }
                        => Self::on_mouse(time, window, event),

                    WindowEvent::Touch { .. }
                        => Self::on_touch(time, window, event),

                    _ => vec![],
                };
                if window.closing.get() {
                    self.window_del(&window_id);
                    return true;
                }
            },
            None => {
                warn!(self.logger, "Received event for unknown window id {window_id:?}")
            }
        }
        false
    }

    pub fn window_del (&self, window_id: &WindowId) -> () {
        self.windows.borrow_mut().remove(&window_id);
    }

    fn on_window <'b> (time: u64, window: &WinitHostWindow, event: WindowEvent<'b>) -> Vec<WinitEvent> {
        match event {
            WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                window.closing.set(true);
                vec![
                    WinitEvent::DeviceRemoved { device: WinitVirtualDevice, }
                ]
            }
            WindowEvent::Resized(psize) => {
                let scale_factor = window.window.scale_factor();
                let mut wsize    = window.scale.borrow_mut();
                let (pw, ph): (u32, u32) = psize.into();
                wsize.physical_size = (pw as i32, ph as i32).into();
                wsize.scale_factor  = scale_factor;
                window.resized.set(Some(wsize.physical_size));
                vec![
                    WinitEvent::Resized { size: wsize.physical_size, scale_factor, }
                ]
            }
            WindowEvent::Focused(focus) => {
                vec![
                    WinitEvent::Focus(focus)
                ]
            }
            WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size, } => {
                let mut wsize = window.scale.borrow_mut();
                wsize.scale_factor = scale_factor;
                let (pw, ph): (u32, u32) = (*new_inner_size).into();
                window.resized.set(Some((pw as i32, ph as i32).into()));
                let size = (pw as i32, ph as i32).into();
                let scale_factor = wsize.scale_factor;
                vec![
                    WinitEvent::Resized { size, scale_factor }
                ]
            }
            _ => vec![]
        }
    }

    fn on_keyboard <'b> (time: u64, window: &WinitHostWindow, event: WindowEvent<'b>)
        -> Vec<WinitEvent>
    {
        match event {
            WindowEvent::KeyboardInput { input, .. } => {
                let KeyboardInput { scancode, state, .. } = input;
                window.rollover.set(match state {
                    ElementState::Pressed  => window.rollover.get() + 1,
                    ElementState::Released => window.rollover.get().checked_sub(1).unwrap_or(0)
                });
                let count = window.rollover.get();
                vec![
                    WinitEvent::Keyboard { time, key: scancode, count, state }
                ]
            }
            _ => vec![]
        }
    }

    fn on_mouse <'b> (time: u64, window: &WinitHostWindow, event: WindowEvent<'b>)
        -> Vec<WinitEvent>
    {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                let size = window.scale.clone();
                let logical_position = position.to_logical(window.scale.borrow().scale_factor);
                vec![
                    WinitEvent::PointerPosition { time, size, logical_position }
                ]
            }
            WindowEvent::MouseWheel { delta, .. } => {
                vec![
                    WinitEvent::PointerAxis { time, delta }
                ]
            }
            WindowEvent::MouseInput { state, button, .. } => {
                vec![
                    WinitEvent::PointerButton { time, button, state, is_x11: window.is_x11 }
                ]
            },
            _ => vec![]
        }
    }

    fn on_touch <'b> (time: u64, window: &WinitHostWindow, event: WindowEvent<'b>)
        -> Vec<WinitEvent>
    {
        let size  = window.scale.clone();
        let scale = window.scale.borrow().scale_factor;
        match event {
            WindowEvent::Touch(Touch { phase: TouchPhase::Started, location, id, .. }) => {
                let location = location.to_logical(scale);
                vec![
                    WinitEvent::TouchDown { size, time, location, id }
                ]
            },
            WindowEvent::Touch(Touch { phase: TouchPhase::Moved, location, id, .. }) => {
                let location = location.to_logical(scale);
                vec![
                    WinitEvent::TouchMotion { size, time, location, id }
                ]
            },
            WindowEvent::Touch(Touch { phase: TouchPhase::Ended, location, id, .. }) => {
                let location = location.to_logical(scale);
                vec![
                    WinitEvent::TouchMotion { size, time, location, id },
                    WinitEvent::TouchUp { time, id }
                ]
            },
            WindowEvent::Touch(Touch { phase: TouchPhase::Cancelled, id, .. }) => {
                vec![
                    WinitEvent::TouchCancel { time, id }
                ]
            },
            _ => vec![]
        }
    }

    pub fn input_added (&mut self, _name: &str) -> Result<()> {
        Ok(())
    }

    pub fn output_added (
        &mut self, _name: &str, screen: usize, width: i32, height: i32
    ) -> Result<()> {
        let window = WinitHostWindow::new(
            &self.logger,
            &self.events.borrow(),
            &make_context(&self.logger, &self.egl_context)?,
            &format!("Output {screen}"),
            width,
            height,
            screen
        )?;
        let window_id = window.id();
        self.windows.borrow_mut().insert(window_id, window);
        Ok(())
    }

}

pub enum WinitEvent {
    Focus(bool),
    TouchUp { time: u64, id: u64 },
    TouchMotion { time: u64, id: u64, size: Rc<RefCell<WindowSize>>, location: LogicalPosition<f64> },
    TouchDown { time: u64, id: u64, size: Rc<RefCell<WindowSize>>, location: LogicalPosition<f64> },
    TouchCancel { time: u64, id: u64 },
    Resized { size: Size<i32, Physical>, scale_factor: f64 },
    PointerPosition { time: u64, size: Rc<RefCell<WindowSize>>, logical_position: LogicalPosition<f64> },
    PointerButton { time: u64, button: MouseButton, state: ElementState, is_x11: bool },
    PointerAxis { time: u64, delta: MouseScrollDelta },
    Keyboard { time: u64, key: u32, count: u32, state: ElementState },
    DeviceRemoved { device: WinitVirtualDevice },
}

#[derive(Debug)]
pub enum WinitHostError {
    WindowClosed,
}

impl std::fmt::Display for WinitHostError {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for WinitHostError {}

fn make_renderer (logger: &slog::Logger, egl: &EGLContext) -> Result<smithay::backend::renderer::gles2::Gles2Renderer> {
    let egl = make_context(logger, egl)?;
    Ok(unsafe { smithay::backend::renderer::gles2::Gles2Renderer::new(egl, logger.clone()) }?)
}

fn make_context (logger: &slog::Logger, egl: &EGLContext) -> Result<EGLContext> {
    Ok(EGLContext::new_shared_with_config(egl.display(), egl, GlAttributes {
        version: (3, 0), profile: None, vsync: true, debug: cfg!(debug_assertions),
    }, Default::default(), logger.clone())?)
}

/// A window created by Winit, displaying a compositor output
#[derive(Debug)]
pub struct WinitHostWindow {
    logger: slog::Logger,

    /// The host window
    pub window:   WinitWindow,
    /// The current window title
    pub title: String,
    /// The current window size
    pub size:  (u32, u32),
    /// The current window scaling
    pub scale: Rc<RefCell<WindowSize>>,

    /// Which viewport is rendered to this window
    pub screen:   usize,
    /// The wayland output
    pub output:   smithay::output::Output,

    /// Count of currently pressed keys
    pub rollover: Cell<u32>,
    /// Is this winit window hosted under X11 (as opposed to a Wayland session?)
    pub is_x11:   bool,
    /// The drawing surface
    pub surface:  Rc<EGLSurface>,
    /// Whether a new size has been specified, to apply on next render
    pub resized:  Rc<Cell<Option<smithay::utils::Size<i32, smithay::utils::Physical>>>>,
    /// Whether the window is closing
    pub closing:  Cell<bool>,
}

/// Build a host window
impl<'a> WinitHostWindow {

    /// Create a new host window
    pub fn new (
        logger: &slog::Logger,
        events: &EventLoop<()>,
        egl:    &EGLContext,
        title:  &str,
        width:  i32,
        height: i32,
        screen: usize
    ) -> Result<Self> {

        // Determine the window dimensions
        let (w, h, hz, subpixel) = (width, height, 60_000, Subpixel::Unknown);

        // Create a new compositor output matching the window
        let output = smithay::output::Output::new(title.to_string(), PhysicalProperties {
            size: (w, h).into(), subpixel, make: "Smithay".into(), model: "Winit".into()
        }, logger.clone());

        // Set the output's mode
        output.change_current_state(
            Some(OutputMode { size: (w, h).into(), refresh: hz }), None, None, None
        );

        // Build the host window
        let window = Self::build(logger, events, title, width, height)?;

        // Store the window's inner size
        let (w, h): (u32, u32) = window.inner_size().into();

        Ok(Self {
            logger:   logger.clone(),
            closing:  Cell::new(false),
            rollover: Cell::new(0),
            is_x11:   window.wayland_surface().is_none(),
            screen,
            output,
            surface:  Self::surface(logger, egl, &window)?,
            size:  (w, h),
            scale: Rc::new(RefCell::new(WindowSize {
                physical_size: (w as i32, h as i32).into(),
                scale_factor:  window.scale_factor(),
            })),
            window,
            resized:  Rc::new(Cell::new(None)),
            title:    title.into(),
        })
    }

    /// Get the window id
    pub fn id (&self) -> WindowId {
        self.window.id()
    }

    /// Build the window
    fn build (
        logger: &slog::Logger,
        events: &EventLoop<()>,
        title:  &str,
        width:  i32,
        height: i32
    ) -> Result<WinitWindow> {

        debug!(logger, "Building Winit window: {title} ({width}x{height})");

        let window = WindowBuilder::new()
            .with_inner_size(LogicalSize::new(width, height))
            .with_title(title)
            .with_visible(true)
            .build(events)?;

        Ok(window)

    }

    /// Obtain the window surface (varies on whether winit is running in wayland or x11)
    fn surface (
        logger: &slog::Logger,
        egl:    &EGLContext,
        window: &WinitWindow
    ) -> Result<Rc<EGLSurface>> {
        debug!(logger, "Setting up Winit window: {window:?}");
        debug!(logger, "Created EGL context for Winit window");
        let surface = if let Some(surface) = window.wayland_surface() {
            Self::surface_wl(logger, &egl, window.inner_size().into(), surface)?
        } else if let Some(xlib_window) = window.xlib_window().map(XlibWindow) {
            Self::surface_x11(logger, &egl, xlib_window)?
        } else {
            unreachable!("No backends for winit other then Wayland and X11 are supported")
        };
        let _ = egl.unbind()?;
        Ok(Rc::new(surface))
    }

    /// Obtain the window surface when running in wayland
    fn surface_wl (
        logger:          &slog::Logger,
        egl:             &EGLContext,
        (width, height): (i32, i32),
        surface:         *mut std::os::raw::c_void
    ) -> Result<EGLSurface> {
        debug!(logger, "Using Wayland backend for Winit window");
        Ok(EGLSurface::new(
            egl.display(),
            egl.pixel_format().unwrap(),
            egl.config_id(),
            unsafe {
                wayland_egl::WlEglSurface::new_from_raw(surface as *mut _, width, height)
            }?,
            logger.clone(),
        )?)
    }

    /// Obtain the window surface when running in X11
    fn surface_x11 (
        logger: &slog::Logger,
        egl:    &EGLContext,
        window: XlibWindow
    ) -> Result<EGLSurface> {
        debug!(logger, "Using X11 backend for Winit window {window:?}");
        Ok(EGLSurface::new(
            egl.display(),
            egl.pixel_format().unwrap(),
            egl.config_id(),
            window,
            logger.clone(),
        ).map_err(EGLError::CreationFailed)?)
    }

}

#[cfg(test)]
mod test {

    use crate::{*, layouts::*, engines::winit::*};
    use std::{error::Error, sync::atomic::Ordering};

    #[test]
    fn winit_should_run () -> Result<()> {
        let app = String::from("just a label");
        let engine = Winit::harness()?;
        engine.exit(); // run once then exit
        app.run(engine)?;
        Ok(())
    }

}
