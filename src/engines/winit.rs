//! # Winit platform
//!
//! This platform renders an interface to one or more windows created with `winit`.

use crate::{*, widgets::*};

use slog::{debug, warn, crit};

use std::{
    rc::Rc,
    cell::{Cell, RefCell, RefMut},
    sync::{Arc, atomic::AtomicBool},
    time::{Instant},
    collections::HashMap,
    //marker::PhantomData
};

use smithay::{
    output::{PhysicalProperties, Subpixel, Mode},
    backend::{
        egl::{
            Error as EGLError, EGLContext, EGLSurface,
            native::XlibWindow,
            context::GlAttributes,
            display::EGLDisplay
        },
        renderer::{ImportDma, ImportEgl},
        winit::{
            Error as WinitError,
            WindowSize,
            WinitEvent,
            WinitVirtualDevice,
            WinitKeyboardInputEvent,
            WinitMouseMovedEvent, WinitMouseWheelEvent, WinitMouseInputEvent,
            WinitTouchStartedEvent, WinitTouchMovedEvent, WinitTouchEndedEvent, WinitTouchCancelledEvent
        }
    },
    backend::{
        input::{
            InputEvent,
        },
    },
    reexports::{
        winit::{
            dpi::LogicalSize,
            event::{Event, WindowEvent, ElementState, KeyboardInput, Touch, TouchPhase},
            event_loop::{ControlFlow, EventLoop as WinitEventLoop},
            platform::unix::WindowExtUnix,
            window::{WindowId, WindowBuilder, Window as WinitWindow},
        },
    }
};

pub type Unit = f32;

impl<'a, X> Engine<Winit<'a>> for X
where
    X: Input<Winit<'a>, &'a [Rect<2, f32>]> + Output<Winit<'a>, bool>
{
    fn done (&self) -> bool {
        false
    }

    fn run (mut self, mut context: Winit<'a>) -> Result<Self> {
        let (display, event) = context.init()?;
        let state = &mut self;
        loop {
            if !context.running.fetch_and(true, Ordering::Relaxed) {
                break
            }

            // Respond to user input
            if let Err(e) = self.handle(context) {
                crit!(context.logger, "Update error: {e}");
                break
            }

            // Render display
            if let Err(e) = self.render(&mut context) {
                crit!(context.logger, "Render error: {e}");
                break
            }

            // Flush display/client messages
            display.borrow_mut().flush_clients()?;

            // Dispatch state to next event loop tick
            events.borrow_mut().dispatch(Some(Duration::from_millis(1)), &mut self)?;
        }

        Ok(self)
    }
}

pub struct Winit {
    logger:        slog::Logger,
    running:       Arc<AtomicBool>,
    display:       Rc<RefCell<smithay::reexports::wayland_server::Display<Self>>>,
    events:        Rc<RefCell<smithay::reexports::calloop::EventLoop<'static, Self>>>,
    started:       Cell<Option<Instant>>,
    windows:       Rc<RefCell<HashMap<WindowId, WinitHostWindow>>>,
    winit_events:  Rc<RefCell<WinitEventLoop<()>>>,
    egl_context:   smithay::backend::egl::EGLContext,
    egl_display:   smithay::backend::egl::display::EGLDisplay,
    renderer:      Rc<RefCell<smithay::backend::renderer::gles2::Gles2Renderer>>,
    frame:         smithay::backend::renderer::gles2::Gles2Frame<'static>,
    shm:           smithay::wayland::shm::ShmState,
    dmabuf_state:  smithay::wayland::dmabuf::DmabufState,
    out_manager:   smithay::wayland::output::OutputManagerState,
}

impl Winit {

    /// Initialize winit engine
    fn new (logger: &slog::Logger, display: &smithay::reexports::wayland_server::DisplayHandle) -> Result<Self> {

        debug!(logger, "Starting Winit engine");

        // Create the Winit event loop
        let winit_events = WinitEventLoop::new();

        // Create a null window to host the EGLDisplay
        let window = std::sync::Arc::new(WindowBuilder::new()
            .with_inner_size(LogicalSize::new(16, 16))
            .with_title("Charlie Null")
            .with_visible(false)
            .build(&winit_events)
            .map_err(WinitError::InitFailed)?);

        // Create the renderer and EGL context
        let egl_display = EGLDisplay::new(window, logger.clone()).unwrap();
        let egl_context = EGLContext::new_with_config(&egl_display, GlAttributes {
            version: (3, 0), profile: None, vsync: true, debug: cfg!(debug_assertions),
        }, Default::default(), logger.clone())?;
        let mut renderer = make_renderer(logger, &egl_context)?;

        // Init dmabuf support
        renderer.bind_wl_display(&display)?;
        let mut dmabuf_state = smithay::wayland::dmabuf::DmabufState::new();

        Ok(Winit {
            logger:        logger.clone(),
            windows:       Rc::new(RefCell::new(HashMap::new())),
            renderer:      Rc::new(RefCell::new(renderer)),
            frame:         None
            shm:           smithay::wayland::shm::ShmState::new::<Self, _>(&display, vec![], logger.clone()),
            out_manager:   smithay::wayland::output::OutputManagerState::new_with_xdg_output::<Self>(&display),
            running:       Arc::new(AtomicBool::new(true)),
            started:       Cell::new(None),
            winit_events:  Rc::new(RefCell::new(winit_events)),
            egl_display,
            egl_context,
            dmabuf_state,
        })
    }

    fn start (self) -> (Rc<RefCell<Display>>, Rc<RefCell<EventLoop>>) {

        // Listen for events
        let display = self.display.clone();
        let fd = display.borrow_mut().backend().poll_fd().as_raw_fd();
        self.events.borrow().handle().insert_source(
            Generic::new(fd, Interest::READ, Mode::Level),
            move |_, _, state| {
                display.borrow_mut().dispatch_clients(state)?;
                Ok(PostAction::Continue)
            }
        )?;

        // Create a socket
        let socket = ListeningSocketSource::new_auto(self.logger.clone()).unwrap();
        let socket_name = socket.socket_name().to_os_string();

        // Listen for new clients
        let socket_logger  = self.logger.clone();
        let mut socket_display = self.display.borrow().handle();
        self.events.borrow().handle().insert_source(socket, move |client, _, _| {
            debug!(socket_logger, "New client {client:?}");
            socket_display.insert_client(
                client.try_clone().expect("Could not clone socket for engine dispatcher"),
                Arc::new(ClientState)
            ).expect("Could not insert client in engine display");
        })?;
        std::env::set_var("WAYLAND_DISPLAY", &socket_name);

        // Run main loop
        (self.display.clone(), self.events.clone())

    }

    fn logger (&self) -> slog::Logger {
        self.logger.clone()
    }

    fn renderer (&self) -> RefMut<smithay::backend::renderer::gles2::Gles2Renderer> {
        self.renderer.borrow_mut()
    }

    /// Render to each host window
    fn render (app: &mut Self) -> Result<()> {
        let windows = app.engine().windows.clone();
        for (_, output) in windows.borrow().iter() {
            if let Some(size) = output.resized.take() {
                output.surface.resize(size.w, size.h, 0, 0);
            }
            app.engine().renderer().bind(output.surface.clone())?;
            let size = output.surface.get_size().unwrap();
            app.render(&output.output, &size, output.screen)?;
            output.surface.swap_buffers(None)?;
        }
        Ok(())
    }

    /// Dispatch input events from the host window to the hosted root widget.
    fn update (app: &mut Self) -> Result<()> {

        let engine = app.engine();
        let mut closed = false;
        if engine.started.get().is_none() {
            //let event = InputEvent::DeviceAdded { device: WinitVirtualDevice };
            //callback(0, WinitEvent::Input(event));
            engine.started.set(Some(Instant::now()));
        }
        let started = &engine.started.get().unwrap();
        let logger = engine.logger.clone();
        let winit_events = engine.winit_events.clone();
        winit_events.borrow_mut().run_return(|event, _target, control_flow| {
            //debug!(self.logger, "{target:?}");
            match event {
                Event::RedrawEventsCleared => {
                    *control_flow = ControlFlow::Exit;
                }
                Event::RedrawRequested(_id) => {
                    //callback(0, WinitEvent::Refresh);
                }
                Event::WindowEvent { window_id, event } => {
                    closed = engine.window_update(&window_id, event)
                }
                _ => {}
            }
        });

        if closed {
            Err(WinitHostError::WindowClosed.into())
        } else {
            Ok(())
        }

    }

    fn dmabuf_state (&mut self) -> &mut smithay::wayland::dmabuf::DmabufState {
        &mut self.dmabuf_state
    }

    fn shm_state (&self) -> &smithay::wayland::shm::ShmState {
        &self.shm
    }

}

impl<'a> Winit<'a> {

    pub fn window_add (&self, window: WinitHostWindow) -> () {
        let window_id = window.id();
        self.windows.borrow_mut().insert(window_id, window);
    }

    pub fn window_update <'b> (&self, window_id: &WindowId, event: WindowEvent<'b>) -> bool {
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
                vec![WinitEvent::Input(InputEvent::DeviceRemoved { device: WinitVirtualDevice, })]
            }
            WindowEvent::Resized(psize) => {
                let scale_factor = window.window.scale_factor();
                let mut wsize    = window.scale.borrow_mut();
                let (pw, ph): (u32, u32) = psize.into();
                wsize.physical_size = (pw as i32, ph as i32).into();
                wsize.scale_factor  = scale_factor;
                window.resized.set(Some(wsize.physical_size));
                vec![WinitEvent::Resized { size: wsize.physical_size, scale_factor, }]
            }
            WindowEvent::Focused(focus) => {
                vec![WinitEvent::Focus(focus)]
            }
            WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size, } => {
                let mut wsize = window.scale.borrow_mut();
                wsize.scale_factor = scale_factor;
                let (pw, ph): (u32, u32) = (*new_inner_size).into();
                window.resized.set(Some((pw as i32, ph as i32).into()));
                let size = (pw as i32, ph as i32).into();
                let scale_factor = wsize.scale_factor;
                vec![WinitEvent::Resized { size, scale_factor }]
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
                    ElementState::Pressed
                        => window.rollover.get() + 1,
                    ElementState::Released
                        => window.rollover.get().checked_sub(1).unwrap_or(0)
                });
                let event = WinitKeyboardInputEvent {
                    time, key: scancode, count: window.rollover.get(), state,
                };
                vec![WinitEvent::Input(InputEvent::Keyboard { event })]
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
                let event = WinitMouseMovedEvent { time, size, logical_position };
                vec![WinitEvent::Input(InputEvent::PointerMotionAbsolute { event })]
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let event = WinitMouseWheelEvent { time, delta };
                vec![WinitEvent::Input(InputEvent::PointerAxis { event })]
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let event = WinitMouseInputEvent { time, button, state, is_x11: window.is_x11 };
                vec![WinitEvent::Input(InputEvent::PointerButton { event }) ]
            },
            _ => vec![]
        }
    }

    fn on_touch <'b> (time: u64, window: &WinitHostWindow, event: WindowEvent<'a>)
        -> Vec<WinitEvent>
    {
        let mut events = vec![];
        let size   = window.scale.clone();
        let scale  = window.scale.borrow().scale_factor;
        match event {
            WindowEvent::Touch(Touch { phase: TouchPhase::Started, location, id, .. }) => {
                let location = location.to_logical(scale);
                let event    = WinitTouchStartedEvent { size, time, location, id };
                events.push(WinitEvent::Input(InputEvent::TouchDown { event }));
            }
            WindowEvent::Touch(Touch { phase: TouchPhase::Moved, location, id, .. }) => {
                let location = location.to_logical(scale);
                let event    = WinitTouchMovedEvent { size, time, location, id };
                events.push(WinitEvent::Input(InputEvent::TouchMotion { event }));
            }
            WindowEvent::Touch(Touch { phase: TouchPhase::Ended, location, id, .. }) => {
                let location = location.to_logical(scale);
                let event    = WinitTouchMovedEvent { size, time, location, id };
                events.push(WinitEvent::Input(InputEvent::TouchMotion { event }));
                let event    = WinitTouchEndedEvent { time, id };
                events.push(WinitEvent::Input(InputEvent::TouchUp { event }));
            }
            WindowEvent::Touch(Touch { phase: TouchPhase::Cancelled, id, .. }) => {
                let event    = WinitTouchCancelledEvent { time, id };
                events.push(WinitEvent::Input(InputEvent::TouchCancel { event }));
            }
            _ => {}
        };
        events
    }

    fn input_added (&mut self, name: &str) -> Result<()> {
        Ok(())
    }

    fn output_added (
        &mut self, name: &str, screen: usize, width: i32, height: i32
    ) -> Result<()> {
        let window = WinitHostWindow::new(
            &self.logger,
            &self.winit_events.borrow(),
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
    pub size:  Point<2, u32>,
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
        events: &WinitEventLoop<()>,
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
            Some(Mode { size: (w, h).into(), refresh: hz }), None, None, None
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
            window,
            size:  Point([w, h]),
            scale: Rc::new(RefCell::new(WindowSize {
                physical_size: (w as i32, h as i32).into(),
                scale_factor:  window.scale_factor(),
            })),
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
        events: &WinitEventLoop<()>,
        title:  &str,
        width:  i32,
        height: i32
    ) -> Result<WinitWindow> {

        debug!(logger, "Building Winit window: {title} ({width}x{height})");

        let window = WindowBuilder::new()
            .with_inner_size(LogicalSize::new(width, height))
            .with_title(title)
            .with_visible(true)
            .build(events)
            .map_err(WinitError::InitFailed)?;

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
                wegl::WlEglSurface::new_from_raw(surface as *mut _, width, height)
            }.map_err(|err| WinitError::Surface(err.into()))?,
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
