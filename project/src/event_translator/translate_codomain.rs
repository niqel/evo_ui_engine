/// Evento funcional central del sistema, generado por EventTranslator.
/// Este evento ya fue interpretado y está listo para ser consumido por otros actores.
#[derive(Debug, Clone)]
pub enum Event {
    /// Tick funcional (flujo del tiempo)
    Tick,

    /// Evento de entrada de usuario
    KeyPressed(String),
    KeyReleased(String),

    /// Movimiento del puntero
    MouseMoved(i32, i32),

    /// Evento de cierre del sistema
    Exit,
}
