//! Application, event loop and window event definitions and implementations.
//!
//! - [**Event**](./enum.Event.html) - the defualt application event type.
//! - [**WindowEvent**](./struct.WindowEvent.html) - events related to a single window.
//! - [**SimpleWindowEvent**](./struct.WindowEvent.html) - a stripped-back, simplified,
//!   newcomer-friendly version of the **raw**, low-level winit event.

use geom::{self, Point2, Vector2};
use state;
use std::path::PathBuf;
use window;
use winit;
use App;

pub use winit::{
    ElementState, KeyboardInput, ModifiersState, MouseButton, MouseScrollDelta, Touch, TouchPhase,
    VirtualKeyCode as Key,
};

/// Event types that are compatible with the nannou app loop.
pub trait LoopEvent: From<Update> {
    /// Produce a loop event from the given winit event.
    fn from_winit_event(winit::Event, &App) -> Option<Self>;
}

/// Update event
#[derive(Clone, Debug)]
pub struct Update {
    /// The duration since the last update was emitted.
    ///
    /// The first update's delta will be the time since the given `model` function returned.
    pub since_last: state::time::Duration,
    /// The duration since the start of the app loop.
    ///
    /// Specifically, this is the duration of time since the given `model` function returned.
    pub since_start: state::time::Duration,
}

/// The default application **Event** type.
#[derive(Clone, Debug)]
pub enum Event {
    /// A window-specific event has occurred for the window with the given Id.
    ///
    /// This event is portrayed both in its "raw" form (the **winit::WindowEvent**) and its
    /// simplified, new-user-friendly form **SimpleWindowEvent**.
    WindowEvent {
        id: window::Id,
        raw: winit::WindowEvent,
        simple: Option<SimpleWindowEvent>,
    },
    /// A device-specific event has occurred for the device with the given Id.
    DeviceEvent(winit::DeviceId, winit::DeviceEvent),
    /// A timed update alongside the duration since the last update was emitted.
    ///
    /// The first update's delta will be the time since the `model` function returned.
    Update(Update),
    /// The application has been awakened.
    Awakened,
    /// The application has been suspended or resumed.
    ///
    /// The parameter is true if app was suspended, and false if it has been resumed.
    Suspended(bool),
}

/// The nannou window event type.
///
/// The **simple** field offers a stripped-back, simplified, newcomer-friendly version of the
/// **raw**, low-level winit event.
#[derive(Clone, Debug)]
pub struct WindowEvent {
    /// A simplified, interpreted version of the `raw` `winit::WindowEvent` emitted via winit.
    ///
    /// See the [SimpleWindowEvent](./enum.SimpleWindowEvent.html)
    pub simple: Option<SimpleWindowEvent>,
    /// The original event type produced by `winit`.
    pub raw: winit::WindowEvent,
}

/// A simplified version of winit's `WindowEvent` type to make it easier to get started.
///
/// All co-ordinates and dimensions are DPI-agnostic scalar values.
///
/// Co-ordinates for each window are as follows:
///
/// - `(0.0, 0.0)` is the centre of the window.
/// - positive `x` points to the right, negative `x` points to the left.
/// - positive `y` points upwards, negative `y` points downwards.
/// - positive `z` points into the screen, negative `z` points out of the screen.
#[derive(Clone, Debug, PartialEq)]
pub enum SimpleWindowEvent {
    /// The window has been moved to a new position.
    Moved(Point2<geom::scalar::Default>),

    /// The given keyboard key was pressed.
    KeyPressed(Key),

    /// The given keyboard key was released.
    KeyReleased(Key),

    /// The mouse moved to the given x, y position.
    MouseMoved(Point2<geom::scalar::Default>),

    /// The given mouse button was dragged to the given x, y position.
    MouseDragged(Point2<geom::scalar::Default>, MouseButton),

    /// The given mouse button was pressed.
    MousePressed(MouseButton),

    /// The given mouse button was released.
    MouseReleased(MouseButton),

    /// The mouse entered the window.
    MouseEntered,

    /// The mouse exited the window.
    MouseExited,

    /// A mouse wheel movement or touchpad scroll occurred.
    MouseWheel(MouseScrollDelta, TouchPhase),

    /// The window was resized to the given dimensions.
    Resized(Vector2<geom::scalar::Default>),

    /// A file at the given path was hovered over the window.
    HoveredFile(PathBuf),

    /// A file at the given path was dropped onto the window.
    DroppedFile(PathBuf),

    /// A file at the given path that was hovered over the window was cancelled.
    HoveredFileCancelled,

    /// Received a touch event.
    Touch {
        phase: TouchPhase,
        position: Point2<geom::scalar::Default>,
        id: u64,
    },

    /// Touchpad pressure event.
    ///
    /// At the moment, only supported on Apple forcetouch-capable macbooks.
    /// The parameters are: pressure level (value between 0 and 1 representing how hard the touchpad
    /// is being pressed) and stage (integer representing the click level).
    TouchpadPressure { pressure: f32, stage: i64 },

    /// The window gained or lost focus.
    ///
    /// The parameter is true if the window has gained focus, and false if it has lost focus.
    Focused(bool),

    /// The window was closed and is no longer stored in the `App`.
    Closed,
}

impl SimpleWindowEvent {
    /// Produce a simplified, new-user-friendly version of the given `winit::WindowEvent`.
    ///
    /// This strips rarely needed technical information from the event type such as information
    /// about the source device, scancodes for keyboard events, etc to make the experience of
    /// pattern matching on window events nicer for new users.
    ///
    /// This also interprets the raw pixel positions and dimensions of the raw event into a
    /// dpi-agnostic scalar value where (0, 0, 0) is the centre of the screen with the `y` axis
    /// increasing in the upwards direction.
    ///
    /// If the user requires this extra information, they should use the `raw` field of the
    /// `WindowEvent` type rather than the `simple` one.
    pub fn from_winit_window_event(
        event: winit::WindowEvent,
        win_w: f64,
        win_h: f64,
    ) -> Option<Self> {
        use self::SimpleWindowEvent::*;

        // Translate the coordinates from top-left-origin-with-y-down to centre-origin-with-y-up.
        //
        // winit produces input events in pixels, so these positions need to be divided by the
        // width and height of the window in order to be DPI agnostic.
        let tw = |w: f64| w as geom::scalar::Default;
        let th = |h: f64| h as geom::scalar::Default;
        let tx = |x: f64| (x - win_w / 2.0) as geom::scalar::Default;
        let ty = |y: f64| (-(y - win_h / 2.0)) as geom::scalar::Default;

        let event = match event {
            winit::WindowEvent::Resized(new_size) => {
                let (new_w, new_h) = new_size.into();
                let x = tw(new_w);
                let y = th(new_h);
                Resized(Vector2 { x, y })
            }

            winit::WindowEvent::Moved(new_pos) => {
                let (new_x, new_y) = new_pos.into();
                let x = tx(new_x);
                let y = ty(new_y);
                Moved(Point2 { x, y })
            }

            // TODO: Should separate the behaviour of close requested and destroyed.
            winit::WindowEvent::CloseRequested |
            winit::WindowEvent::Destroyed => Closed,

            winit::WindowEvent::DroppedFile(path) => DroppedFile(path),

            winit::WindowEvent::HoveredFile(path) => HoveredFile(path),

            winit::WindowEvent::HoveredFileCancelled => HoveredFileCancelled,

            winit::WindowEvent::Focused(b) => Focused(b),

            winit::WindowEvent::CursorMoved {
                position, ..
            } => {
                let (x, y) = position.into();
                let x = tx(x);
                let y = ty(y);
                MouseMoved(Point2 { x, y })
            }

            winit::WindowEvent::CursorEntered { .. } => MouseEntered,

            winit::WindowEvent::CursorLeft { .. } => MouseExited,

            winit::WindowEvent::MouseWheel { delta, phase, .. } => MouseWheel(delta, phase),

            winit::WindowEvent::MouseInput { state, button, .. } => match state {
                ElementState::Pressed => MousePressed(button),
                ElementState::Released => MouseReleased(button),
            },

            winit::WindowEvent::Touch(winit::Touch {
                phase,
                location,
                id,
                ..
            }) => {
                let (x, y) = location.into();
                let x = tx(x);
                let y = ty(y);
                let position = Point2 { x, y };
                Touch {
                    phase,
                    position,
                    id,
                }
            }

            winit::WindowEvent::TouchpadPressure {
                pressure, stage, ..
            } => TouchpadPressure { pressure, stage },

            winit::WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                Some(key) => match input.state {
                    ElementState::Pressed => KeyPressed(key),
                    ElementState::Released => KeyReleased(key),
                },
                None => return None,
            },

            winit::WindowEvent::AxisMotion { .. }
            | winit::WindowEvent::Refresh
            | winit::WindowEvent::ReceivedCharacter(_)
            | winit::WindowEvent::HiDpiFactorChanged(_) => {
                return None;
            }
        };

        Some(event)
    }
}

impl LoopEvent for Event {
    /// Convert the given `winit::Event` to a nannou `Event`.
    fn from_winit_event(event: winit::Event, app: &App) -> Option<Self> {
        let event = match event {
            winit::Event::WindowEvent { window_id, event } => {
                let windows = app.windows.borrow();
                let (win_w, win_h) = match windows.get(&window_id) {
                    None => (0.0, 0.0), // The window was likely closed, these will be ignored.
                    Some(window) => {
                        match window.surface.window().get_inner_size() {
                            None => (0.0, 0.0),
                            Some(size) => size.into(),
                        }
                    }
                };
                let raw = event.clone();
                let simple = SimpleWindowEvent::from_winit_window_event(event, win_w, win_h);
                Event::WindowEvent {
                    id: window_id,
                    raw,
                    simple,
                }
            }
            winit::Event::DeviceEvent { device_id, event } => Event::DeviceEvent(device_id, event),
            winit::Event::Awakened => Event::Awakened,
            winit::Event::Suspended(b) => Event::Suspended(b),
        };
        Some(event)
    }
}

impl From<Update> for Event {
    fn from(update: Update) -> Self {
        Event::Update(update)
    }
}
