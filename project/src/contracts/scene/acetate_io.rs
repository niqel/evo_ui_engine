// acetate_io.rs for scene module

// TODO: implement
/// Representa el estado funcional de entrada/salida de un acetate.
#[derive(Debug, Clone)]
pub struct AcetateIO {
    pub content: Option<String>,
    pub focus: bool,
    pub status: AcetateStatus,
}


/// Estado lógico de un acetate (activo, oculto, error, etc.)
#[derive(Debug, Clone)]
pub enum AcetateStatus {
    Ready,
    Hidden,
    Disabled,
    Error(String),
}
