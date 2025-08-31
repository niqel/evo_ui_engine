// Tipos de eventos del motor (contrato central, puro e inmutable)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    // Tiempo / sistema
    Tick,
    Exit,
    WindowResized { width: u32, height: u32 },

    // Teclado
    KeyPressed(String),
    KeyReleased(String),
    TextInput(String),

    // Puntero
    MouseMoved(i32, i32),
    MouseDown { button: MouseButton, x: i32, y: i32 },
    MouseUp   { button: MouseButton, x: i32, y: i32 },

    // Extensión / placeholder
    Custom(&'static str),
}

impl Event {
    pub fn is_tick(&self) -> bool { matches!(self, Event::Tick) }
    pub fn is_exit(&self) -> bool { matches!(self, Event::Exit) }
    pub fn is_resize(&self) -> bool { matches!(self, Event::WindowResized { .. }) }
}
