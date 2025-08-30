//! codominio module for event_interpreter

// TODO: implement
/// Representa un evento funcional interno que puede ser interpretado por el sistema.
#[derive(Debug, Clone)]
pub enum InternalEvent {
    /// Tick funcional proveniente del TimeTicker
    Tick,

    /// Entrada de usuario (teclado, ratón, etc.)
    Input {
        kind: InputKind,
    },

    /// Evento de control general (como salir del programa)
    SystemExit,
}

/// Tipos de entrada posibles dentro de `Input`
#[derive(Debug, Clone)]
pub enum InputKind {
    KeyPressed(String),
    KeyReleased(String),
    MouseMoved(i32, i32),
    MouseClicked,
    // puedes extender según necesites
}
