// Tipos de eventos del motor (contrato central, puro e inmutable)
use super::EventKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    // Tiempo / sistema
    Tick(crate::actors::ticker::TickCodomain),
    Exit,
    WindowResized { width: u32, height: u32 },

    // Teclado
    KeyPressed(String),
    KeyReleased(String),
    TextInput(String),

    // Puntero
    MouseMoved(i32, i32),
    MouseClicked,
    MouseDown { button: MouseButton, x: i32, y: i32 },
    MouseUp   { button: MouseButton, x: i32, y: i32 },
}

impl Event {
    pub fn kind(&self) -> EventKind {
        match self {
            Event::Tick(_) => EventKind::Tick,
            Event::Exit => EventKind::Exit,
            Event::WindowResized { .. } => EventKind::WindowResized,
            Event::KeyPressed(_) => EventKind::KeyPressed,
            Event::KeyReleased(_) => EventKind::KeyReleased,
            Event::TextInput(_) => EventKind::TextInput,
            Event::MouseMoved(_, _) => EventKind::MouseMoved,
            Event::MouseClicked => EventKind::MouseClicked,
            Event::MouseDown { .. } => EventKind::MouseDown,
            Event::MouseUp { .. } => EventKind::MouseUp,
        }
    }

    pub fn is_tick(&self) -> bool { self.kind() == EventKind::Tick }
    pub fn is_exit(&self) -> bool { self.kind() == EventKind::Exit }
    pub fn is_resize(&self) -> bool { self.kind() == EventKind::WindowResized }

    /// Compara dos eventos por variante, ignorando payloads dinámicos (p. ej. Tick).
    pub fn same_kind(a: &Event, b: &Event) -> bool {
        matches!(
            (a, b),
            (Event::Tick(_), Event::Tick(_))
                | (Event::Exit, Event::Exit)
                | (Event::WindowResized { .. }, Event::WindowResized { .. })
                | (Event::KeyPressed(_), Event::KeyPressed(_))
                | (Event::KeyReleased(_), Event::KeyReleased(_))
                | (Event::TextInput(_), Event::TextInput(_))
                | (Event::MouseMoved(_, _), Event::MouseMoved(_, _))
                | (Event::MouseClicked, Event::MouseClicked)
                | (Event::MouseDown { .. }, Event::MouseDown { .. })
                | (Event::MouseUp { .. }, Event::MouseUp { .. })
        )
    }
}
