//! codominio module for translator

// TODO: implement
/// Representa una proyección visual funcional de la Scene,
/// lista para ser pasada al Renderer (y renderizada con Vello).
#[derive(Debug, Clone)]
pub struct Snapshot {
    pub layers: Vec<SnapshotLayer>,
}

/// Una capa visual dentro del Snapshot.
/// Puede representar un acetate, una forma, un texto, etc.
#[derive(Debug, Clone)]
pub struct SnapshotLayer {
    pub z_index: i32,
    pub area: Rect,
    pub style: SnapshotStyle,
}

/// Información visual para pintar una capa.
#[derive(Debug, Clone)]
pub struct SnapshotStyle {
    pub fill_color: Color,
    pub border_color: Color,
    pub border_thickness: f32,
    pub text: Option<String>,
}

/// Área rectangular (reutilizable en Scene y Snapshot).
#[derive(Debug, Clone)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

/// Color RGBA funcional.
#[derive(Debug, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
