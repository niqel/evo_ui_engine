/// Evento crudo del sistema (host/platform) todavía sin interpretar.
#[derive(Debug, Clone)]
pub enum SystemEvent {
    TickSignal,

    KeyDown(String),
    KeyUp(String),

    MouseMove(i32, i32),

    ExitRequested,
    // Si luego agregas MouseDown/MouseUp/Scroll, extiende aquí.
}
