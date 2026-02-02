use super::interpret_domain::SystemEvent;
use super::interpret_codomain::{InternalEvent, InputKind};

/// Actor funcional que interpreta eventos crudos del sistema
/// y los convierte en eventos funcionales internos.
pub struct EventRouter;

impl EventRouter {
    pub fn interpret(input: SystemEvent) -> InternalEvent {
        match input {
            SystemEvent::TickSignal => InternalEvent::Tick,

            SystemEvent::KeyDown(key) => InternalEvent::Input {
                kind: InputKind::KeyPressed(key),
            },

            SystemEvent::KeyUp(key) => InternalEvent::Input {
                kind: InputKind::KeyReleased(key),
            },

            SystemEvent::MouseMove(x, y) => InternalEvent::Input {
                kind: InputKind::MouseMoved(x, y),
            },

            SystemEvent::ExitRequested => InternalEvent::SystemExit,
        }
    }
}
