//! dominio module for event_interpreter

// TODO: implement
/// Representa un evento crudo del sistema, sin interpretar aún.
#[derive(Debug, Clone)]
pub enum SystemEvent {
    /// Evento de sistema de tiempo (por ejemplo, Tick externo)
    TickSignal,

    /// Tecla presionada
    KeyDown(String),

    /// Tecla liberada
    KeyUp(String),

    /// Movimiento del ratón
    MouseMove(i32, i32),

    /// Cierre de ventana u orden de salida
    ExitRequested,
}
