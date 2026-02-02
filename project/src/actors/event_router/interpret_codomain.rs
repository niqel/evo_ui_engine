/// Kinds de entrada ya “normalizados”
#[derive(Debug, Clone)]
pub enum InputKind {
    KeyPressed(String),
    KeyReleased(String),
    MouseMoved(i32, i32),
    MouseClicked, // placeholder si quieres mantenerlo por ahora
}

/// Evento interno del motor tras interpretar el evento crudo
#[derive(Debug, Clone)]
pub enum InternalEvent {
    Tick,
    Input { kind: InputKind },
    SystemExit,
}
